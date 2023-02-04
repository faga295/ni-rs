use std::collections::HashMap;

pub fn resolve_args(args: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut options: HashMap<String, Vec<String>> = HashMap::from([
        (String::from("name"), vec![]),
        (String::from("value"), vec![]),
    ]);
    for arg in args {
        if arg.starts_with('-') {
            if let Some(vec) = options.get_mut("name") {
                vec.push(arg.to_string())
            }
        } else {
            if let Some(vec) = options.get_mut("value") {
                vec.push(arg.to_string())
            }
        }
    }
    options
}
