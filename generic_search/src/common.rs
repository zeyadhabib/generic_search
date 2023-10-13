use colored::*;

pub fn print_error_message(message: &str) {
    println!("{} {}", "[ERROR]".red().bold(), message.red().bold());
}

pub fn print_local_file_match(file_name: &str) {
    println!("{} {}", "[LOCAL][FIL] ".green(), file_name.green());
}

pub fn print_local_directory_match(file_name: &str) {
    println!("{} {}", "[LOCAL][DIR] ".green().blink(), file_name.green().blink());
}

pub fn print_remote_file_match(file_name: &str) {
    println!("{} {}", "[REMOTE][FIL] ".green(), file_name.green());
}

pub fn print_remote_directory_match(file_name: &str) {
    println!("{} {}", "[REMOTE][DIR] ".green().blink(), file_name.green().blink());
}

