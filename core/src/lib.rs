mod logger;
mod util;
mod whitelist;

extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use scharschbot_core::config::load::load_config;
use scharschbot_core::config::config_format::Config;
use scharschbot_core::websocket::websocket::connect_ws;
use scharschbot_core::plugin::logger::{info, error};
use scharschbot_core::events::mc_events::{player_join, player_leave, player_chat, player_death, player_advancement};
use scharschbot_core::jni_utils::set_vm;
use crate::util::{extract_death_message, extract_message, extract_player, get_server_name, extract_advancement};

static mut CONFIG: Option<Config> = None;

pub static mut CLASS: Option<JClass<'static>> = None;

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onInitialize(env: JNIEnv, class: JClass<'static>) {
    match env.get_java_vm() {
        Ok(vm) => set_vm(vm),
        Err(err) => {
            error(format!("Error getting java vm: {}", err));
        }
    };

    unsafe {
        CLASS = Some(class);
    }

    logger::set();
    info(format!("Loading Config!"));
    match load_config() {
        Ok(_) => {}
        Err(err) => {
            error(format!("Error loading config: {}", err));
            return;
        }
    };

    info("Connecting to websocket!".to_string());

    match connect_ws() {
        Ok(_) => info(format!("Connected to websocket!")),
        Err(err) => error(format!("Error connecting to websocket: {}", err)),
    };
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerJoin(_: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(&event);
    player_join(name, get_server_name());
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerLeave(_: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(&event);
    player_leave(name, get_server_name());
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerChat(_: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(&event);
    let message = extract_message(&event);
    player_chat(name, message, get_server_name());
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerDeath(_: JNIEnv, _: JClass, event: JObject) {
    let name = extract_player(&event);
    let death_message = extract_death_message(&event);
    player_death(name, death_message, get_server_name());

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerAdvancement(_: JNIEnv, _: JClass, event: JObject) {
    let name = extract_player(&event);
    let advancement = extract_advancement(&event);
    player_advancement(name, advancement, get_server_name());

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onShutdown(_: JNIEnv, _: JClass, _: JObject) {
    // TODO: Close websocket
}