use crate::scrape::{html_to_course, html_to_course_abstracts};

mod sample;
mod scrape;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn fetch_test() {
    println!("fetch_test");

    // let url = "https://www.ocw.titech.ac.jp/index.php?module=General&action=T0100&GakubuCD=1&lang=JA";
    // let rc = reqwest::blocking::get(url).unwrap();
    // let res = rc.text().unwrap();

    let res = sample::get_sample_main();

    let courses = html_to_course_abstracts(&res);

    println!("{:?}", courses.len());
    println!("{:?}", courses[1]);

    let detail = sample::get_sample_sub();
    let detail = html_to_course(&detail);

    println!("{:?}", detail);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetch_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
