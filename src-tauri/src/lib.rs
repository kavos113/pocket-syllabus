use crate::scrape::{html_to_course, html_to_course_abstracts};
use sqlx::sqlite::SqliteError;
use sqlx::SqlitePool;
use std::process::Command;
use std::time;
use tauri::async_runtime::block_on;
use tauri::path::BaseDirectory;
use tauri::{Emitter, Manager, State};

mod database;
mod sample;
mod scrape;

use crate::database::{CourseListItem, CourseResponse, SearchQuery};
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

    let courses = html_to_course_abstracts(res.as_ref());

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

    block_on(database::insert_course(&sqlite_pool, &detail)).unwrap();

    (*app).emit("fetch_status", "finish insert").unwrap();
}

#[tauri::command]
async fn fetch(
    sqlite_pool: State<'_, SqlitePool>,
    app: State<'_, tauri::AppHandle>,
) -> Result<(), ()> {
    println!("fetch");

    (*app).emit("fetch_status", "Start Fetching").unwrap();

    let urls = vec![
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=1&lang=JA", //理学院
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=4&lang=JA", // 情報
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=2&lang=JA", //工学院
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=3&lang=JA", // 物質
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=5&lang=JA", // 生命
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=6&lang=JA", // 環社
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=10&lang=JA", // 初年専門
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=11&lang=JA", // 共通
        "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=7&lang=JA", // 教養
    ];

    for url in urls {
        (*app)
            .emit("fetch_status", "Fetching Main".to_string())
            .unwrap();
        let rc = reqwest::get(url).await.unwrap();
        let res = rc.text().await.unwrap();
        tokio::time::sleep(time::Duration::from_secs(10)).await;

        let courses = html_to_course_abstracts(res.as_ref());

        let length = courses.len();
        let mut now = 0;

        (*app)
            .emit("fetch_status", format!("Left: {} courses", length))
            .unwrap();

        for course in courses {
            now += 1;
            (*app)
                .emit(
                    "fetch_status",
                    format!(
                        "Scraping: {}/{} courses: {} {}",
                        now, length, course.department, course.title.title
                    ),
                )
                .unwrap();

            println!(
                "Scraping: {}/{} courses: {}",
                now, length, course.title.title
            );

            if course.code.is_empty() {
                continue;
            }

            let check = database::check_sylbs_update(
                &sqlite_pool,
                &course.code,
                &course.title.title,
                &course.sylbs_update,
            )
            .await
            .unwrap();

            if check {
                continue;
            }

            let rc = reqwest::get(course.title.url.as_str()).await.unwrap();
            let mut detail = html_to_course(rc.text().await.unwrap().as_ref());
            detail.url = course.title.url;
            detail.sylbs_update = course.sylbs_update;
            tokio::time::sleep(time::Duration::from_secs(10)).await;

            database::insert_course(&sqlite_pool, &detail)
                .await
                .unwrap();

            (*app)
                .emit(
                    "fetch_status",
                    format!("Finished: {}/{} courses: {}", now, length, detail.title),
                )
                .unwrap();
        }

        (*app).emit("fetch_status", "finish insert").unwrap();
    }
    Ok(())
}

#[tauri::command]
async fn search_courses(
    sqlite_pool: State<'_, SqlitePool>,
    search_query: SearchQuery,
) -> Result<Vec<CourseListItem>, ()> {
    let courses = database::search_courses(&*sqlite_pool, search_query).await;

    Ok(courses)
}

#[tauri::command]
async fn get_course(sqlite_pool: State<'_, SqlitePool>, id: i32) -> Result<CourseResponse, ()> {
    let course = database::get_course(&*sqlite_pool, id).await;

    Ok(course)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            fetch_test,
            fetch,
            search_courses,
            get_course
        ])
        .setup(|app| {
            let app_dir = app.path().app_data_dir().unwrap();

            std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

            let db_path = app_dir.join("database.db");
            let db_path = db_path.to_str().unwrap();

            // let db_path = app
            //     .path()
            //     .resolve("database.db", BaseDirectory::Resource)
            //     .unwrap();
            // let db_path = db_path.as_path().to_str().unwrap();
            // let db_path = if db_path.starts_with("\\\\?\\") {
            //     &db_path[4..]
            // } else {
            //     db_path
            // };
            println!("db_path: {}", db_path);
            let sqlite_pool = block_on(database::create_sqlite_pool(db_path)).unwrap();
            block_on(database::migrate(&sqlite_pool)).unwrap();

            app.manage(sqlite_pool);
            app.manage(app.app_handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
