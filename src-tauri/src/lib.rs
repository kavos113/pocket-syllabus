use crate::scrape::{html_to_course, html_to_course_abstracts};
use sqlx::{Sqlite, SqlitePool};
use tauri::async_runtime::block_on;
use tauri::Manager;
use tauri::State;

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
fn fetch_test(sqlite_pool: State<'_, SqlitePool>) {
    println!("fetch_test");

    // let url = "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=1&lang=JA";
    // let rc = reqwest::blocking::get(url).unwrap();
    // let res = rc.text().unwrap();

    let res = sample::get_sample_main();

    let courses = html_to_course_abstracts((&res).as_ref());

    println!("{:?}", courses.len());
    println!("{:?}", courses[1]);

    let detail = sample::get_sample_sub();
    let detail = html_to_course((&detail).as_ref());

    println!("{:?}", detail);

    block_on(database::insert_course(&*sqlite_pool, &detail)).unwrap();
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
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
