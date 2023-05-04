use std::fs::File;
use std::io::Write;
use std::path::Path;
use scharschbot_core::jni_utils::get_env;
use serde::{Deserialize, Serialize};
use scharschbot_core::plugin::logger::error;
use crate::handlers::bukkit::get_bukkit;


const WHITELIST_PATH: &str = "whitelist.json";


pub(crate) fn whitelist_add(name: String, uuid: String) -> Result<(), String> {
    let mut whitelist = match get_whitelist() {
        Ok(whitelist) => whitelist,
        Err(_) => {
            return Err("Error getting whitelist".to_string());
        }
    };

    let entry = WhitelistEntry {
        name,
        uuid,
    };

    whitelist.push(entry);

    match save_whitelist(whitelist) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error saving whitelist".to_string())
    }
}

pub(crate) fn whitelist_remove(name: String) -> Result<(), String> {
let mut whitelist = match get_whitelist() {
        Ok(whitelist) => whitelist,
        Err(_) => {
            return Err("Error getting whitelist".to_string());
        }
    };

    whitelist.retain(|entry| entry.name != name);

    match save_whitelist(whitelist) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error saving whitelist".to_string())
    }
}

fn get_whitelist() -> Result<Vec<WhitelistEntry>, ()> {
    let whitelist_path = Path::new(WHITELIST_PATH);
    let whitelist_file = match File::open(&whitelist_path) {
        Ok(file) => file,
        Err(_) => {
            print_whitelist_not_found();
            return Err(());
        }
    };

    let whitelist: Vec<WhitelistEntry> = match serde_json::from_reader(whitelist_file) {
        Ok(whitelist) => whitelist,
        Err(e) => {
            error(format!("Error parsing whitelist file: {}", e));
            return Err(());
        }
    };

    Ok(whitelist)
}

fn save_whitelist(whitelist: Vec<WhitelistEntry>) -> Result<(), ()> {
    let whitelist_path = Path::new(WHITELIST_PATH);
    let mut whitelist_file = match File::create(&whitelist_path) {
        Ok(file) => file,
        Err(e) => {
            error(format!("Error creating whitelist file: {}", e));
            return Err(());
        }
    };
    let whitelist_string = match serde_json::to_string(&whitelist) {
        Ok(string) => string,
        Err(e) => {
            error(format!("Error serializing whitelist: {}", e));
            return Err(());
        }
    };
    match whitelist_file.write(whitelist_string.as_bytes()) {
        Ok(_) => {
            match reload_whitelist() {
                Ok(_) => Ok(()),
                Err(e) => {
                    error(format!("Error reloading whitelist: {:?}", e));
                    return Err(());
                }
            }
        },
        Err(e) => {
            error(format!("Error writing whitelist file: {}", e));
            Err(())
        }
    }
}


fn reload_whitelist() -> Result<(), ()> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(e) => {
            error(format!("Error getting env: {:?}", e));
            return Err(());
        }
    };

    let mut bukkit = match get_bukkit(){
        Ok(bukkit) => bukkit,
        Err(()) => return Err(())
    };

    match env.call_static_method(bukkit, "reloadWhitelist", "()V", &[]) {
        Ok(_) => Ok(()),
        Err(e) => {
            error(format!("Error calling reloadWhitelist: {:?}", e));
            Err(())
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WhitelistEntry {
    uuid: String,
    name: String,
}

fn print_whitelist_not_found() {
    let msg = r#"
    ╭─────────────────────────────────────────────────────────────────╮
    │                                                                 │
    │                    Whitelist file not found!                    │
    │          The Scharschbot whitelist function won't work          │
    │                                                                 │
    ╰─────────────────────────────────────────────────────────────────╯"#;
    error(msg);
}