use super::args::args_to_string;
pub fn print_command(manager: &str, arg: &str, args: &Vec<String>) {
    println!("> {} {}{}", manager, arg, args_to_string(&args[1..]))
}
