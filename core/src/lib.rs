mod logger;
mod util;

extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use std::ops::Deref;
use scharschbot_core::config::load::load_config;
use scharschbot_core::websocket::websocket::connect_ws;
use scharschbot_core::plugin::logger::{info, error};
use scharschbot_core::events::mc_events::{player_join, player_leave, player_chat};
use scharschbot_core::{set_class};
use crate::util::{extract_message, extract_player, get_server_name};

static mut CONFIG: Option<scharschbot_core::config::config::Config> = None;

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onInitialize(mut env: JNIEnv, class: JClass) {
    set_class(class);
    logger::set();
    info(&mut env, format!("Loading Config!"));
    let config = match load_config(){
        Ok(config) => {
            unsafe {
                CONFIG = Some(config.clone());
            }
            config
        },
        Err(err) => {
            error(&mut env,format!("Error loading config: {}", err));
            return;
        }
    };

    info(&mut env,"Connecting to websocket!".to_string());

    match connect_ws(config){
        Ok(_) => info(&mut env,format!("Connected to websocket!")),
        Err(err) => error(&mut env,format!("Error connecting to websocket: {}", err)),
    };
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerJoin(mut env: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(&mut env, event);
    player_join(name, get_server_name());

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerLeave(mut env: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(&mut env, event);
    player_leave(name, get_server_name());

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerChat(mut env: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(&mut env, unsafe { JObject::from_raw(event.as_ref().deref().clone())});
    let message = extract_message(&mut env, event);
    player_chat(name, message, get_server_name());

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerDeath(mut _env: JNIEnv, _class: JClass, _event: JObject) {

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerAdvancement(mut _env: JNIEnv, _class: JClass, _event: JObject) {

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onShutdown(_env: JNIEnv, _class: JClass, _event: JObject) {
    // TODO: Close websocket
}
