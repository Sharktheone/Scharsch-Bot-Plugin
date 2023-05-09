//TODO: Bot => Server: SendCommand  ✅
use jni::objects::JValue;
use scharschbot_core::jni_utils::{call_static_stacking, get_env, JBOOLEAN, JniFn, JSTRING, make_signature};
use crate::handlers::bukkit::get_bukkit;

pub(crate) fn send_command(command: String) -> Result<(), String> {
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

    let command_string = match env.new_string(command) {
        Ok(string) => string,
        Err(e) => {
            return Err(format!("Error creating command string: {}", e));
        }
    };

    let command_arg = JValue::Object(&command_string);

    let console_sender_fns = [
        JniFn {
            name: "getConsoleSender",
            input: &[],
            output: "Lorg/bukkit/command/ConsoleCommandSender;",
            args: &[],
        }
    ];

    let console_sender = call_static_stacking(&bukkit, &console_sender_fns);
    if console_sender.is_null() {
        return Err("Error getting console sender".to_string());
    }

    let console_sender_arg = JValue::Object(&console_sender);

    let success = match env.call_static_method(&bukkit, "dispatchCommand", "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)Z", &[console_sender_arg, command_arg]) {
        Ok(name) => {
            match name.z() {
                Ok(name) => name,
                Err(e) => {
                    return Err(format!("Error converting dispatchCommand output: {}", e));
                }
            }
        }
        Err(e) => {
            return Err(format!("Error calling dispatchCommand: {}", e));
        }
    };
    if !success {
        return Err("Error dispatching command".to_string());
    }

    Ok(())
}