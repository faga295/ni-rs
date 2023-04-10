use prompts::{select::SelectPrompt, Prompt};
use serde_json::Value;
use std::{collections::HashSet, fs::File, io::Read};
pub async fn detect_package_manager() -> String {
    let lock_files = ["package-lock.json", "yarn.lock", "pnpm-lock.yaml"];
    let manager_list = ["npm", "yarn", "pnpm"];
    let mut detected_list = HashSet::new();
    let mut manager = String::from("npm");
    for (index, lock_file) in lock_files.into_iter().enumerate() {
        if let Ok(_) = File::open(lock_file) {
            println!("file:{}", lock_file);
            detected_list.insert(manager_list[index]);
            manager = manager_list[index].to_string();
        };
    }
    let mut package_json = String::new();
    File::open("package.json")
        .expect("No package.json file")
        .read_to_string(&mut package_json)
        .expect("Can't read package.json");
    if let Ok(parsed) = serde_json::from_str::<Value>(&package_json) {
        if let Some(scripts) = parsed["scripts"].as_object() {
            for (_, value) in scripts {
                if let Some(script) = value.as_str() {
                    for (_, manager_item) in manager_list.into_iter().enumerate() {
                        if script.contains(manager_item) {
                            detected_list.insert(manager_item);
                            manager = manager_item.to_string();
                        }
                    }
                }
            }
        }
    }
    if detected_list.len() != 1 {
        let data = vec!["npm", "yarn", "pnpm"];
        let mut prompt = SelectPrompt::new("Choose your favourite package manager", data);

        match prompt.run().await {
            Ok(Some(s)) => return s.to_string(),
            Ok(None) => println!("Prompt was aborted!"),
            Err(e) => println!("Some kind of crossterm error happened: {:?}", e),
        }
    }
    manager
}
