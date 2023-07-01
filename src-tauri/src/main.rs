#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::{fs, env};
use ttswd_gui::modinfo::get_workshop_mod;
use tauri::{Manager, WindowEvent};
use scraper::{Html, Selector};
use serde_json::{Value, json};
use regex::Regex;

lazy_static! {
    static ref WORKSHOP_FOLDER: String = format!(
        "{}/Documents/My Games/Tabletop Simulator/Mods/Workshop",
        dirs::home_dir().unwrap().as_path().to_str().unwrap()
    );
}

#[tauri::command(async)]
fn download_mod(id: &str, author: &str, img: &str) -> bool {
    let mut mod_json: Value = match get_workshop_mod(id) {
        Ok(m) => m,
        Err(e) => {
            println!("[ERROR] Failed to get mod info. {}", e);
            return false;
        }
    };

    mod_json["id"] = json!(id);
    mod_json["author"] = json!(author);
    mod_json["img"] = json!(img);

    let save_path = format!("{}/{}.json", *WORKSHOP_FOLDER, id);
    if let Err(e) = fs::write(&save_path, mod_json.to_string()) {
        println!("[ERROR] Failed to write save file. {}", e);
    } else {
        println!("[DONE] Mod saved at /Mods/Workshop/{}.json. You may now play it!", id);
    }

    true
}

#[tauri::command(async)]
fn remove_mod(id: &str) -> bool {
    if let Ok(_) = fs::remove_file(format!("{}/{}.json", *WORKSHOP_FOLDER, id)) {
        true
    } else {
        false
    }
}

#[tauri::command(async)]
fn search_workshop(query: &str, sort: &str, library: &str, page: u32) -> String {
    if library == "online" {
        search_online(query, sort, page)
    } else {
        get_local_mods()
    }
}

fn search_online(query: &str, sort: &str, page: u32) -> String {
    let body = ureq::get(
        &format!(
            "https://steamcommunity.com/workshop/browse/?appid=286160&searchtext={}&childpublishedfileid=0&browsesort={}&section=readytouseitems&created_date_range_filter_start=0&created_date_range_filter_end=0&updated_date_range_filter_start=0&updated_date_range_filter_end=0&p={}",
            query, sort, page
        )
    )//.set("Cookie", COOKIE)
    .call().unwrap().into_string().unwrap();

    let mut items: Vec<Value> = vec![];
    let doc = Html::parse_document(&body);

    let id_sel = Selector::parse("a").unwrap();
    let item_sel = Selector::parse("div.workshopItem").unwrap();
    let title_sel = Selector::parse("div.workshopItemTitle").unwrap();
    let author_sel = Selector::parse("div.workshopItemAuthorName").unwrap();
    let img_sel = Selector::parse("img").unwrap();
    let pages_sel = Selector::parse(".workshopBrowsePagingControls").unwrap();
    
    let pages = match doc.select(&pages_sel).next() {
        Some(p) => {
            let text = p.text().collect::<String>().trim().to_string();
            if text.is_empty() {
                String::from("0")
            } else {
                let pages = text.split(char::from(160)).collect::<Vec<&str>>();
                pages.get(pages.len() - 2).unwrap().to_string()
            }
        },
        None => String::from("0")
    };
    let pages = pages.replace(",", "").parse::<u32>().unwrap();
    
    for e in doc.select(&item_sel) {
        let href: &str = e.select(&id_sel).next().unwrap().value().attr("href").unwrap();
        let title = e.select(&title_sel).next().unwrap().text().collect::<String>();
        let author = e.select(&author_sel).next().unwrap().text().collect::<String>();
        let img = e.select(&img_sel).next().unwrap().value().attr("src").unwrap();

        let re = Regex::new(r"\?id=([^&]+)").unwrap();
        let id = re.captures(href).unwrap().get(1).unwrap().as_str();

        let mut is_downloaded = false;
        if let Ok(_) = fs::metadata(format!("{}/{}.json", *WORKSHOP_FOLDER, id)) {
            is_downloaded = true;
        }

        items.push(json!({
            "id": id,
            "title": title,
            "author": author,
            "img": img,
            "downloaded": is_downloaded
        }))
    }

    json!({
        "pages": pages,
        "items": items
    }).to_string()
}

fn get_local_mods() -> String {
    let mut items: Vec<Value> = vec![];
    
    if let Ok(files) = fs::read_dir(format!("{}", *WORKSHOP_FOLDER)) {
        for path in files {
            let entry = path.unwrap();
            let content = fs::read_to_string(entry.path()).unwrap();
            let data: Value = serde_json::from_str(&content).unwrap();
            let file_name = entry.file_name().into_string().unwrap();

            if file_name == "WorkshopFileInfos.json" {
                continue;
            }

            items.push(json!({
                "id": data["id"].as_str().unwrap_or(&file_name[..(file_name.len()-5)]),
                "title": data["GameMode"].as_str().unwrap_or(data["GameMode"].as_str().unwrap()),
                "author": data["author"].as_str().unwrap_or("Unknown author"),
                "img": data["img"].as_str().unwrap_or("https://i.imgur.com/6eEa9bB.png"),
                "downloaded": true
            }));
        }
    }

    json!({
        "items": items
    }).to_string()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let Some(window) = app.get_window("main") else {
                return Ok(())
            };

            window.on_window_event(|event| {
                match event {
                    WindowEvent::Resized(..) => {
                        std::thread::sleep(std::time::Duration::from_millis(1))
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![download_mod, remove_mod, search_workshop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
