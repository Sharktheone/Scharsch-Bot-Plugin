use jni::objects::{JClass, JObject};
use jni::signature::ReturnType;
use scharschbot_core::jni_utils::{call_stacking, convert_string, convert_string_or, get_env, JniFn, JSTRING};
use scharschbot_core::config::load::CONFIG;
use scharschbot_core::plugin::logger::error;

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
            name: "getDisplay",
            input: &[],
            output: "io.papermc.paper.advancement.AdvancementDisplay",
            args: &[],
        },
        JniFn {
            name: "title",
            input: &[],
            output: "net.kyori.adventure.text.Component",
            args: &[],
        }
    ];
    let title_obj = call_stacking(event, &fns);

    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            error("Error getting env");
            return convert_string_or(&title_obj, "Error getting Advancement");
        }
    };

    let translatable_component = match env.find_class("net/kyori/adventure/text/TranslatableComponent") {
        Ok(class) => class,
        Err(e) => {
            error(format!("Error getting TranslatableComponent class: {}", e));
            return convert_string_or(&title_obj, "Error getting Advancement");
        }
    };

    let is_translatable_obj = match env.is_instance_of(unsafe {JObject::from_raw(title_obj.as_raw())}, unsafe { JClass::from_raw(translatable_component.as_raw())}) {
        Ok(is_translatable) => is_translatable,
        Err(e) => {
            error(format!("Error checking if title is translatable: {}", e));
            return convert_string_or(&title_obj, "Error getting Advancement");
        }
    };

    if is_translatable_obj {
        let key_method = match env.get_method_id(translatable_component, "key", "()Ljava/lang/String;") {
            Ok(method) => method,
            Err(e) => {
                error(format!("Error getting key method: {}", e));
                return convert_string_or(&title_obj, "Error getting Advancement");
            }
        };
        let key_obj = unsafe {
            match env.call_method_unchecked(&title_obj, key_method, ReturnType::Object, &[]) {
                Ok(obj) => match obj.l() {
                    Ok(obj) => obj,
                    Err(e) => {
                        error(format!("Error getting key: {}", e));
                        return convert_string_or(&title_obj, "Error getting Advancement");
                    }
                },
                Err(e) => {
                    error(format!("Error getting key: {}", e));
                    return convert_string_or(&title_obj, "Error getting Advancement");
                }
            }
        };
        return convert_string(&key_obj)
    }

    return convert_string_or(&title_obj, "Error getting Advancement");

}

pub(crate) fn get_server_name() -> String {
    unsafe {
        match CONFIG.as_ref() {
            Some(config) => config.serverid.clone(),
            None => "Unknown".to_string()
        }
    }
}