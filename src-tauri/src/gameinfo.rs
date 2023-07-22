use std::{io::Cursor, error::Error, fs};
use serde_json::Value;
use super::decoder::decode_contents;

pub fn get_workshop_game(game_id: &str) -> Result<Value, Box<dyn Error>> {
    let game_contents: Vec<u8>;
    let cache_file_path = format!("cache/Download_{}", game_id);

    if let Err(_) = fs::metadata("cache") {
        if let Err(e) = fs::create_dir("cache") {
            return Err(format!("Failed to create cache directory. {}", e).into())
        }
    }

    if let Ok(contents) = fs::read(&cache_file_path) {
        game_contents = contents;
        println!("[INFO] Game is cached. Loading from file.")
    } else {
        println!("[INFO] Game is not cached. Requesting game from external server [https://steamworkshopdownloader.io/]...");
    
        let body = ureq::post("https://db.steamworkshopdownloader.io/prod/api/details/file")
            .set("content-type", "application/x-www-form-urlencoded")
            .send(Cursor::new(format!("[{}]", game_id)))?
            .into_string()?;
    
        let parsed: Value = serde_json::from_str(&body)?;
        if let None = parsed[0]["file_url"].as_str() {
            return Err("Download URL for game file not found.".into());
        }
        
        println!("[INFO] Got game {}", parsed[0]["title"]);
        println!("[INFO] Found url {}.", parsed[0]["file_url"]);
        println!("[INFO] Requesting game...");
    
        let response = ureq::get(parsed[0]["file_url"].as_str().unwrap()).call()?;
        let mut buf: Vec<u8> = vec![];

        response.into_reader().read_to_end(&mut buf).unwrap();

        if let Err(e) = fs::write(&cache_file_path, &buf) {
            println!("[WARN] Failed to save file on cache. {}", e);
        }

        game_contents = buf;
    }

    match decode_contents(game_contents) {
        Ok(m) => Ok(m),
        Err(e) => Err(format!("Failed to decode contents. {}", e).into())
    }
}