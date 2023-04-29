use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use scharschbot_core::jni_utils::{make_signature};
use scharschbot_core::plugin::logger::{set_loggers};

fn log(msg: &str, env: &mut JNIEnv, class: &JObject, method: &str) -> Result<(), String> {
    let msg_str:JObject = env.new_string(msg).unwrap().into();
    let logger = match env.get_field(&class, "logger", make_signature(&"java.util.logging.Logger".to_string())) {
        Ok(logger) => match logger.l() {
            Ok(logger) => logger,
            Err(e) => {
                return Err(format!("Error converting logger to object: {}", e))
            }
        }
        Err(e) => {
            return Err(format!("Error extracting logger: {}", e));
        }
    };

    match env.call_method(logger, method, "(Ljava/lang/String;)V", &[JValue::Object(msg_str.as_ref())]){
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(format!("Error calling logger: {}", e));
        }
    }
}

fn info(msg: &str, env: &mut JNIEnv, class: &JObject) -> Result<(), String> {
    log(msg, env, class, "info")
}
fn warn(msg: &str, env: &mut JNIEnv, class: &JObject) -> Result<(), String> {
    log(msg, env, class, "warn")
}
fn error(msg: &str, env: &mut JNIEnv, class: &JObject) -> Result<(), String> {
    log(msg, env, class, "error")
}

pub fn set() {
    set_loggers(&info, &warn, &error);
}