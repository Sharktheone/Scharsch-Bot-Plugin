use jni::objects::{JObject, JValue};
use scharschbot_core::jni_utils::{get_class, get_env};
use scharschbot_core::plugin::logger::{error_no_env, set_loggers};

static mut LOGGER: Option<JObject> = None;

fn log(msg: &str, method: &str) -> Result<(), String> {
    let logger = unsafe {
        match &LOGGER {
            Some(logger) => logger,
            None => return Err("No logger".to_string()),
        }
    };

    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return Err("No env".to_string()),
    };



    let msg_str: JObject = match env.new_string(msg) {
        Ok(msg_str) => msg_str.into(),
        Err(e) => return Err(format!("Error creating string: {}", e)),
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

    match env.call_method(logger, "log", "(Ljava/util/logging/Level;Ljava/lang/String;)V", &[JValue::Object(&level), JValue::Object(&msg_str)]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error calling logger: {}", e))
    }
}

fn info(msg: &str) -> Result<(), String> {
    log(msg, "info")
}

fn warn(msg: &str) -> Result<(), String> {
    log(msg, "warn")
}

fn error(msg: &str) -> Result<(), String> {
    log(msg, "error")
}

pub fn set() {
    set_loggers(&info, &warn, &error);

    let class = match get_class() {
        Ok(class) => class,
        Err(_) => {
            error_no_env("No class".to_string());
            return;
        },
    };

    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            error_no_env("No env".to_string());
            return;
        },
    };

    let logger = match env.get_field(class, "logger", "Ljava/util/logging/Logger;") {
        Ok(logger) => {
            match logger.l() {
                Ok(logger) => {
                    logger
                }
                Err(e) => {
                    error_no_env(format!("Error converting logger to object: {}", e));
                    return;
                }
            }
        }
        Err(e) => {
            error_no_env(format!("Error extracting logger: {}", e));
            return;
        }
    };

    unsafe {
        LOGGER = Some(logger);
    }
}