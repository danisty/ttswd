#[macro_use]
extern crate lazy_static;

use std::{fs, env};
use ttswd::gameinfo::get_workshop_game;
use serde_json::Value;

lazy_static! {
    static ref WORKSHOP_FOLDER: String = format!(
        "{}/Documents/My Games/Tabletop Simulator/Mods/Workshop",
        dirs::home_dir().unwrap().as_path().to_str().unwrap()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let id: &String = match args.get(1) {
        Some(s) => s,
        None => return println!("[ERROR] Game id not provided. Usage example: ./ttswd-cli.exe 2522804644")
    };

    let game_info: Value = match get_workshop_game(id) {
        Ok(m) => m,
        Err(e) => return println!("[ERROR] Failed to get game info. {}", e)
    };

    let save_path = format!("{}/{}.json", *WORKSHOP_FOLDER, id);
    if let Err(e) = fs::write(&save_path, game_info.to_string()) {
        println!("[ERROR] Failed to write save file. {}", e);
    } else {
        println!("[DONE] Game saved at /Mods/Workshop/{}.json. You may now play it!", id);
    }
}
