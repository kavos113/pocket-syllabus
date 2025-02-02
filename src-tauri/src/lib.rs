use crate::scrape::{html_to_course, html_to_course_abstracts};
use sqlx::SqlitePool;
use tauri::async_runtime::block_on;
use tauri::{Emitter, Manager, State};

mod database;
mod sample;
mod scrape;

pub use scrape::Course;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn fetch_test(sqlite_pool: State<'_, SqlitePool>, app: State<'_, tauri::AppHandle>) {
    println!("fetch_test");

    (*app).emit("fetch_status", "test").unwrap();

    let url =
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=1&lang=JA";
    let rc = reqwest::blocking::get(url).unwrap();
    let res = rc.text().unwrap();

    // let res = sample::get_sample_main();

    let courses = html_to_course_abstracts((&res).as_ref());

    println!("{:?}", courses.len());
    println!("{:?}", courses[1]);

    //let detail = sample::get_sample_sub();
    //let detail = html_to_course((&detail).as_ref());

    let check = block_on(database::check_sylbs_update(
        &*sqlite_pool,
        &courses[1].code,
        &courses[1].title.title,
        &courses[1].sylbs_update,
    ))
    .unwrap();

    if check {
        (*app).emit("fetch_status", "already fetched").unwrap();
        return;
    }

    let rc = reqwest::blocking::get(courses[1].title.url.as_str()).unwrap();
    let mut detail = html_to_course(rc.text().unwrap().as_ref());
    detail.url = courses[1].title.url.clone();
    detail.sylbs_update = courses[1].sylbs_update.clone();

    println!("{:?}", detail);

    (*app).emit("fetch_status", "finish scrape").unwrap();

    block_on(database::insert_course(&*sqlite_pool, &detail)).unwrap();

    (*app).emit("fetch_status", "finish insert").unwrap();
}

#[tauri::command]
fn fetch(sqlite_pool: State<'_, SqlitePool>, app: State<'_, tauri::AppHandle>) {
    println!("fetch");

    (*app).emit("fetch_status", "Start Fetching").unwrap();

    let url =
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=1&lang=JA";
    let rc = reqwest::blocking::get(url).unwrap();
    let res = rc.text().unwrap();

    let courses = html_to_course_abstracts(res.as_ref());

    let length = courses.len();
    let mut now = 0;

    (*app)
        .emit("fetch_status", format!("Left: {} courses", length))
        .unwrap();

    for course in courses {
        let check = block_on(database::check_sylbs_update(
            &sqlite_pool,
            &course.code,
            &course.title.title,
            &course.sylbs_update,
        ))
        .unwrap();

        if check {
            now += 1;
            (*app)
                .emit(
                    "fetch_status",
                    format!(
                        "Finished: {}/{} courses: {}",
                        now, length, course.title.title
                    ),
                )
                .unwrap();
            continue;
        }

        let rc = reqwest::blocking::get(course.title.url.as_str()).unwrap();
        let mut detail = html_to_course(rc.text().unwrap().as_ref());
        detail.url = course.title.url;
        detail.sylbs_update = course.sylbs_update;

        block_on(database::insert_course(&sqlite_pool, &detail)).unwrap();

        now += 1;

        (*app)
            .emit(
                "fetch_status",
                format!("Finished: {}/{} courses: {}", now, length, detail.title),
            )
            .unwrap();
    }

    (*app).emit("fetch_status", "finish insert").unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sqlite_pool = block_on(database::create_sqlite_pool("./database.db")).unwrap();
    block_on(database::migrate(&sqlite_pool)).unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetch_test])
        .setup(|app| {
            app.manage(sqlite_pool);
            app.manage(app.app_handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
