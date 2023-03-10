use ini::ini;
use std::collections::HashMap;
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn resolve_config() -> HashMap<String, HashMap<String, Option<String>>> {
    let rc_path = get_rc_path();
    let config = ini!(rc_path.to_str().unwrap());
    println!("fas");
    return config;
}

pub fn get_global_agent() -> String {
    let config = resolve_config();
    let mut global_agent: String = String::from("npm");
    if let Some(default) = config.get("default").unwrap().get("globalAgent") {
        global_agent = default.clone().unwrap()
    }
    global_agent
}

pub fn get_default_agent() -> String {
    let config = resolve_config();
    let mut default_agent: String = String::from("prompt");
    if let Some(default) = config.get("default").unwrap().get("defaultAgent") {
        default_agent = default.clone().unwrap()
    }
    default_agent
}

pub fn get_rc_path() -> PathBuf {
    if let Ok(file) = env::var("NI_CONFIG_FILE") {
        return Path::new(&file).to_path_buf();
    }
    let mut home = env::home_dir().expect("Home dir is not found");
    home.push(".nirc");
    home
}
