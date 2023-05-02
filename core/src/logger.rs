use jni::objects::{JObject, JValue};
use scharschbot_core::jni_utils::get_env;
use scharschbot_core::plugin::logger::{set_loggers};
use crate::CLASS;

fn log(msg: &str, method: &str) -> Result<(), String> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return Err("No env".to_string()),
    };

    let class = unsafe {
        match CLASS.as_mut() {
            Some(class) => class,
            None => return Err("No class".to_string()),
        }
    };


    let msg_str:JObject = match env.new_string(msg) {
        Ok(msg_str) => msg_str.into(),
        Err(e) => return Err(format!("Error creating string: {}", e)),
    };

    let logger = match env.get_field(class, "logger", "Ljava/util/logging/Logger;") {
        Ok(logger) => {

            match logger.l() {
                Ok(logger) => {
                    logger
                }
                Err(e) => {
                    return Err(format!("Error converting logger to object: {}", e));
                }
            }
        }
        Err(e) => {
            return Err(format!("Error extracting logger: {}", e));
        }
    };

    let log_level = match method {
        "info" => "INFO",
        "warn" => "WARNING",
        "error" => "SEVERE",
        _ => return Err("Unknown log level".to_string())
    };

    let level = match env.get_static_field("java/util/logging/Level", log_level, "Ljava/util/logging/Level;") {
        Ok(level) => match level.l() {
            Ok(level) => level,
            Err(e) => return Err(format!("Error converting level to object: {}", e))
        }
        Err(e) => return Err(format!("Error getting level: {}", e))
    };

    match env.call_method(logger, "log", "(Ljava/util/logging/Level;Ljava/lang/String;)V", &[JValue::Object(&level), JValue::Object(&msg_str)]){
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error calling logger: {}", e))
    }
}

fn info(msg: &str) -> Result<(), String> {
    log(msg,"info")
}
fn warn(msg: &str) -> Result<(), String> {
    log(msg,"warn")
}
fn error(msg: &str) -> Result<(), String> {
    log(msg,"error")
}

pub fn set() {
    set_loggers(&info, &warn, &error);
}