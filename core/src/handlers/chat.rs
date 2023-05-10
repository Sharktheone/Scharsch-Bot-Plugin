//TODO: Bot => Server: SendChatMessage
//TODO: Bot => Server: SendAdminMessage

use jni::objects::JValue;
use scharschbot_core::events::message::{ERROR, Message, MessageData};
use scharschbot_core::jni_utils::{get_env, JINT, JniFn};
use scharschbot_core::plugin::kyori_adventure::parse_component::parse_component;
use scharschbot_core::websocket::websocket::send;
use crate::handlers::bukkit::get_bukkit;

pub(crate) fn send_message(message: String, is_component: bool) -> Result<(), String> {
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

    let component = match parse_component(message, is_component) {
        Ok(component) => component,
        Err(_) => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    players: None,
                    player: None,
                    uuid: None,
                    reason: None,
                    command: None,
                    message: None,
                    death_message: None,
                    message_is_component: None,
                    advancement: None,
                    password: None,
                    user: None,
                    error: Some("Failed to parse component / text".to_string()),
                    server: None,
                }
            };

            match send(msg) {
                Ok(_) => {},
                Err(e) => {
                    return Err(format!("Error sending error message: {}", e));
                }
            };
            return Err("Error parsing component / text".to_string());
        }
    };

    let broadcast_arg = JValue::Object(&component);

    let _ = match env.call_static_method(&bukkit, "broadcast", "(Lnet/kyori/adventure/text/Component;)I", &[broadcast_arg]) {
        Ok(players_received) => match players_received.i() {
            Ok(players_received) => {
                let players: i32 = players_received.into();
                players
            },
            Err(e) => {
                return Err(format!("Error getting broadcast return value: {}", e));
            }
        },
        Err(e) => {
            return Err(format!("Error calling broadcast: {}", e));
        }
    };

    Ok(())

}