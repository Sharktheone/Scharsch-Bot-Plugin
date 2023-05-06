use jni::objects::JValue;
use scharschbot_core::jni_utils::{call_static_stacking, get_env, JniFn, JSTRING, JVOID};
use scharschbot_core::plugin::kyori_adventure::component::basic_component;
use scharschbot_core::plugin::kyori_adventure::parse_component::parse_component;
use crate::handlers::bukkit::get_bukkit;

pub(crate) fn kick_player(player: String, reason: String, is_component: bool) -> Result<(), String>{
    let env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            return Err("Error getting env".to_string());
        }
    };

    let bukkit = match get_bukkit() {
        Ok(bukkit) => bukkit,
        Err(_) => {
            return Err("Error getting bukkit".to_string());
        }
    };

    let component = match parse_component(reason, is_component){
        Ok(component) => component,
        Err(_) => {
            match basic_component("Failed to parse reason, please contact an admin for more information".to_string()){
                Ok(component) => component,
                Err(_) => {
                    return Err("Error parsing reason".to_string());
                }
            }
        }
    };


    let kick_arg =  JValue::Object(&component);


    let player_string = match env.new_string(player) {
        Ok(string) => string,
        Err(e) => {
            return Err(format!("Error creating player string: {}", e));
        }
    };

    let player_arg = JValue::Object(&player_string);

    let fns = [
        JniFn {
            name: "getPlayer",
            input: &[JSTRING],
            output: "org.bukkit.entity.Player",
            args: &[player_arg],
        },
        JniFn {
            name: "kick",
            input: &["Lnet/kyori/adventure/text/Component;"],
            output: JVOID,
            args: &[kick_arg],
        }
    ];

    call_static_stacking(&bukkit, &fns);
    Ok(())
}