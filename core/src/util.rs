use std::ops::Deref;
use jni::JNIEnv;
use jni::objects::{JObject};
use scharschbot_core::jni_utils::{call_stacking, convert_string, JniFn, JSTRING};

pub(crate) fn extract_player(mut env: &mut JNIEnv, event: JObject) -> String {
    let fns = [
        JniFn {
            name: "getPlayer",
            input: &[],
            output: "org.bukkit.entity.Player",
            args: &[],
        },
        JniFn {
            name: "getName",
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];
    let player_obj = call_stacking(&mut env, event, &fns);

    convert_string(&mut env, player_obj)
}

pub(crate) fn extract_message(mut env: &mut JNIEnv, event: JObject) -> String {
    let fns = [
        JniFn {
            name: "message",
            input: &[],
            output: "net.kyori.adventure.text.Component",
            args: &[],
        },
        JniFn {
            name: "toString",
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];
    let message_obj = call_stacking(&mut env, event, &fns);

    convert_string(&mut env, message_obj)
}