use std::process::exit;

const WHITE: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

pub fn print_notice(msg: &str) {
    eprintln!("{}{}NOTICE:{} {}", BLUE, BOLD, WHITE, msg);
}

pub fn print_error(msg: &str) {
    eprintln!("{}{}ERROR:{} {}", RED, BOLD, WHITE, msg);
}

pub fn print_warn(msg: &str) {
    eprintln!("{}{}WARN:{} {}", YELLOW, BOLD, WHITE, msg);
}

pub fn throw_critical_error(msg: &str) {
    print_error(msg);
    exit(1);
}