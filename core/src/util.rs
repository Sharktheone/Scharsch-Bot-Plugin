use jni::objects::{JObject};
use scharschbot_core::jni_utils::{call_stacking, convert_string, JniFn, JSTRING};
use scharschbot_core::config::load::CONFIG;

pub(crate) fn extract_player(event: &JObject) -> String {
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
    let player_obj = call_stacking(event, &fns);

    convert_string(&player_obj)
}

pub(crate) fn extract_message(event: &JObject) -> String {
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
    let message_obj = call_stacking(event, &fns);

    convert_string(&message_obj)
}

pub(crate) fn extract_death_message(event: &JObject) -> String {
    let fns = [
        JniFn {
            name: "getDeathMessage", // TODO: Find alternative for deprecated method
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];
    let message_obj = call_stacking(event, &fns);

    convert_string(&message_obj)
}

pub(crate) fn extract_advancement(event : &JObject) -> String {
    let fns = [
        JniFn {
            name: "getAdvancement",
            input: &[],
            output: "org.bukkit.advancement.Advancement",
            args: &[],
        },
        JniFn {
            name: "getKey",
            input: &[],
            output: "org.bukkit.NamespacedKey",
            args: &[],
        },
        JniFn {
            name: "getKey",
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];
    let message_obj = call_stacking(event, &fns);

    convert_string(&message_obj)
}

pub(crate) fn get_server_name() -> String {
    unsafe {
        match CONFIG.as_ref() {
            Some(config) => config.serverid.clone(),
            None => "Unknown".to_string()
        }
    }
}