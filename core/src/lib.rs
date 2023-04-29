extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JObject};


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onInitialize(mut env: JNIEnv, class: JClass) {

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerJoin(mut env: JNIEnv, class: JClass, event: JObject) {

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerLeave(mut env: JNIEnv, class: JClass, event: JObject) {

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerChat(mut env: JNIEnv, class: JClass, event: JObject) {

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerDeath(mut env: JNIEnv, class: JClass, event: JObject) {

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onPlayerAdvancement(mut env: JNIEnv, class: JClass, event: JObject) {

}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_plugin_Events_onShutdown(_env: JNIEnv, _class: JClass, _event: JObject) {
    // TODO: Close websocket
}