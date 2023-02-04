use serde_json::Value;
use std::{fs::File, io::Read};
pub fn detect_package_manager() -> String {
    let lock_files = ["package.lock", "yarn.lock", "pnpm-lock.yaml"];
    let manager_list = ["npm", "yarn", "pnpm"];
    let mut manager = String::from("npm");
    for (index, lock_file) in lock_files.into_iter().enumerate() {
        if let Ok(_) = File::open(lock_file) {
            manager = manager_list[index].to_string();
            return manager;
        };
    }
    let mut package_json = String::new();
    File::open("package.json")
        .expect("No package.json file")
        .read_to_string(&mut package_json);
    if let Ok(parsed) = serde_json::from_str::<Value>(&package_json) {
        if let Some(scripts) = parsed["scripts"].as_object() {
            for (_, value) in scripts {
                if let Some(script) = value.as_str() {
                    for (_, manager_item) in manager_list.into_iter().enumerate() {
                        if script.contains(manager_item) {
                            manager = manager_item.to_string();
                            return manager;
                        }
                    }
                }
            }
        }
    }
    manager
}
