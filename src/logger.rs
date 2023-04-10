use colored::*;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn log(prefix: ColoredString, str: &str) {
    control::set_virtual_terminal(true).ok();
    println!("{} {}", prefix, str);
}

pub fn success(str: &str) {
    log(format!("[{PACKAGE_NAME}]").green(), str);
}

pub fn error(str: &str) {
    log(format!("[{PACKAGE_NAME}]").red(), str);
}
