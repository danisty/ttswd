use std::{fs, env};
use ttswd_gui::modinfo::{get_workshop_mod};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mod_id: &String = match args.get(1) {
        Some(s) => s,
        None => return println!("[ERROR] Mod id not provided. Usage example: ./cli 2522804644")
    };

    let mod_info: Value = match get_workshop_mod(mod_id) {
        Ok(m) => m,
        Err(e) => return println!("[ERROR] Failed to get mod info. {}", e)
    };

    let save_path = format!("C:/Users/danis/Documents/My Games/Tabletop Simulator/Mods/Workshop/{}.json", mod_id);
    if let Err(e) = fs::write(&save_path, mod_info.to_string()) {
        println!("[ERROR] Failed to write save file. {}", e);
    } else {
        println!("[DONE] Mod saved at /Mods/Workshop/{}.json. You may now play it!", mod_id);
    }
}
