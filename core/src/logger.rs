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

    let logger = match env.get_field(class, "logger", "Ljava.util.logging.Logger;") {
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

    match env.call_method(logger, method, "(Ljava/lang/String;)V", &[JValue::Object(msg_str.as_ref())]){
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