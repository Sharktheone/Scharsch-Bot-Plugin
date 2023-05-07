use jni::objects::JClass;
use scharschbot_core::jni_utils::get_env;
use scharschbot_core::plugin::logger::error;

pub(crate) fn get_bukkit() -> Result<JClass<'static>, ()> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            error("Error getting env");
            return Err(());
        }
    };

    match env.find_class("org/bukkit/Bukkit") {
        Ok(bukkit) => Ok(bukkit),
        Err(e) => {
            error(format!("Error getting Bukkit class: {:?}", e));
            Err(())
        }
    }
}