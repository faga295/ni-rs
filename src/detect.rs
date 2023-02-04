use std::{fs::File, io::Read};
pub fn detect_package_manager() -> String {
    let lock_files = ["package.lock", "yarn.lock", "pnpm-lock.yaml"];
    let manager_list = ["npm", "yarn", "pnpm"];
    let mut manager = String::from("npm");
    for (index, lock_file) in lock_files.into_iter().enumerate() {
        if let Ok(_) = File::open(lock_file) {
            manager = manager_list[index].to_string()
        };
    }
    let mut package_json = String::new();
    File::open("package.json")
        .expect("No package.json file")
        .read_to_string(&mut package_json);
    manager
}
