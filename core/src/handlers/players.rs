use jni::objects::{JObjectArray, JString, JValue};
use scharschbot_core::jni_utils::{call_static_stacking, get_env, JniFn, JSTRING, JVOID};
use scharschbot_core::plugin::kyori_adventure::component::basic_component;
use scharschbot_core::plugin::kyori_adventure::parse_component::{parse_component, parse_component_to_legacy};
use crate::handlers::bukkit::get_bukkit;
use crate::util::extract_name_from_player;

//TODO: Bot => Server: SendPlayers
//TODO: Bot => Server: KickPlayer          ✅
//TODO: Bot => Server: BanPlayer           ✅
//TODO: Bot => Server: UnbanPlayer         ✅

pub(crate) fn kick_player(player: String, reason: String, is_component: bool) -> Result<(), String> {
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

    let component = match parse_component(reason, is_component) {
        Ok(component) => component,
        Err(_) => {
            match basic_component("Failed to parse reason, please contact the support for more information".to_string()) {
                Ok(component) => component,
                Err(_) => {
                    return Err("Error parsing reason".to_string());
                }
            }
        }
    };


    let kick_arg = JValue::Object(&component);


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

pub(crate) fn ban_player(player: String, reason: String, is_component: bool) -> Result<(), String> {
    let env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            return Err("Error getting env".to_string());
        }
    };

    let component = match parse_component_to_legacy(reason, is_component) {
        Ok(component) => component,
        Err(_) => {
            let obj = match basic_component("Failed to parse reason, please contact the support for more information".to_string()) {
                Ok(component) => component,
                Err(_) => {
                    return Err("Error parsing reason".to_string());
                }
            };
            JString::from(obj)
        }
    };

    let ban_arg = JValue::Object(&component);

    let player_string = match env.new_string(player) {
        Ok(string) => string,
        Err(e) => {
            return Err(format!("Error creating player string: {}", e));
        }
    };

    let player_arg = JValue::Object(&player_string);


    let bukkit = match get_bukkit() {
        Ok(bukkit) => bukkit,
        Err(_) => {
            return Err("Error getting bukkit".to_string());
        }
    };

    let fns = [
        JniFn {
            name: "getPlayer",
            input: &[JSTRING],
            output: "Lorg/bukkit/entity/Player;",
            args: &[player_arg],
        },
        JniFn {
            name: "banPlayer",
            input: &[JSTRING],
            output: "org.bukkit.BanEntry",
            args: &[ban_arg],
        }
    ];

    call_static_stacking(&bukkit, &fns);


    Ok(())
}

pub(crate) fn unban_player(player: String) -> Result<(), String> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            return Err("Error getting env".to_string());
        }
    };

    let player_string = match env.new_string(player) {
        Ok(string) => string,
        Err(e) => {
            return Err(format!("Error creating player string: {}", e));
        }
    };

    let bukkit = match get_bukkit() {
        Ok(bukkit) => bukkit,
        Err(_) => {
            return Err("Error getting bukkit".to_string());
        }
    };

    let ban_list_type = match env.find_class("org/bukkit/BanList$Type") {
        Ok(class) => class,
        Err(e) => {
            return Err(format!("Error getting BanList class: {}", e));
        }
    };

    let ban_list_name_args = env.new_string("NAME").unwrap();

    let ban_list_name = env.call_static_method(ban_list_type, "valueOf", "(Ljava/lang/String;)Lorg/bukkit/BanList$Type;", &[JValue::Object(&ban_list_name_args)]).unwrap().l().unwrap();


    let ban_list_arg = JValue::Object(&ban_list_name);

    let player_arg = JValue::Object(&player_string);

    let pardon_fns = [
        JniFn {
            name: "getBanList",
            input: &["Lorg/bukkit/BanList$Type;"],
            output: "org.bukkit.BanList",
            args: &[ban_list_arg],
        },
        JniFn {
            name: "pardon",
            input: &[JSTRING],
            output: JVOID,
            args: &[player_arg],
        }
    ];

    call_static_stacking(&bukkit, &pardon_fns);

    Ok(())
}

pub(crate) fn send_players() -> Result<Vec<String>, String> {
    let mut env = match get_env() {
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

    let fns = [
        JniFn {
            name: "getOnlinePlayers",
            input: &[],
            output: "java.util.Collection",
            args: &[],
        },
        JniFn{
            name: "toArray",
            input: &[],
            output: "[Ljava/lang/Object;",
            args: &[],
        }
    ];

    let players_obj = call_static_stacking(&bukkit, &fns);

    let players_array:JObjectArray = JObjectArray::from(players_obj);

    let len = match env.get_array_length(&players_array) {
        Ok(len) => len,
        Err(e) => {
            return Err(format!("Error getting array length: {}", e));
        }
    };

    let mut players = Vec::with_capacity(len as usize);

    for i in 0..len {
        let player = match env.get_object_array_element(&players_array, i) {
            Ok(player) => player,
            Err(e) => {
                return Err(format!("Error getting player: {}", e));
            }
        };
        let player = extract_name_from_player(&player);
        players.push(player);
    }

    Ok(players)
}