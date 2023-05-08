mod logger;
mod util;
mod handlers;

extern crate jni;

use std::thread;
use std::time::Duration;
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use scharschbot_core::config::load::load_config;
use scharschbot_core::events::handler::{Handlers, set_handlers};
use scharschbot_core::websocket::websocket::connect_ws;
use scharschbot_core::plugin::logger::{info, error, logger_pump};
use scharschbot_core::events::mc_events::{player_join, player_leave, player_chat, player_death, player_advancement};
use scharschbot_core::jni_utils::{set_class, set_vm};
use crate::util::{extract_death_message, extract_message, extract_player, get_server_name, extract_advancement};
use crate::handlers::whitelist::{whitelist_add, whitelist_remove};

#[no_mangle]
pub extern "C" fn Java_de_scharschbot_plugin_Events_onInitialize(env: JNIEnv, class: JClass<'static>) {
    match env.get_java_vm() {
        Ok(vm) => {
            set_vm(vm);
        },
        Err(err) => {
            error(format!("Error getting java vm: {}", err));
        }
    }
    set_class(class);
    logger::set();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10)); // Wait for the logger_pump to be initialized
        let handlers = Handlers {
            get_players_handler: None,
            kick_player: None,
            ban_player: None,
            unban_player: None,
            send_command: None,
            send_message: None,
            send_admin_message: None,
            add_whitelist: Some(&whitelist_add),
            remove_whitelist: Some(&whitelist_remove),
            whitelisted_players: None
        };
        set_handlers(handlers);

        info("Loading Config!");
        match load_config() {
            Ok(_) => {}
            Err(err) => {
                error(format!("Error loading config: {}", err));
                return;
            }
        };

        info("Connecting to websocket!");

        match connect_ws() {
            Ok(_) => {},
            Err(err) => error(format!("Error connecting to websocket: {}", err)),
        };
    });

    logger_pump();
}

#[no_mangle]
pub extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerJoin(_: JNIEnv, _: JClass, event: JObject) {
    let name = extract_player(&event);
    player_join(name, get_server_name());
}

#[no_mangle]
pub extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerLeave(_: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(&event);
    player_leave(name, get_server_name());
}

#[no_mangle]
pub extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerChat(_: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(&event);
    let message = extract_message(&event);
    player_chat(name, message, get_server_name());
}

#[no_mangle]
pub extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerDeath(_: JNIEnv, _: JClass, event: JObject) {
    let name = extract_player(&event);
    let death_message = extract_death_message(&event);
    player_death(name, death_message, get_server_name());

}

#[no_mangle]
pub extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerAdvancement(_: JNIEnv, _: JClass, event: JObject) {
    let name = extract_player(&event);
    let advancement = match extract_advancement(&event) {
        Ok(advancement) => advancement,
        Err(_) => return
    };
    player_advancement(name, advancement, get_server_name());
}

#[no_mangle]
pub extern "C" fn Java_de_scharschbot_plugin_Events_onShutdown(_: JNIEnv, _: JClass, _: JObject) {
    // TODO: Close websocket
}