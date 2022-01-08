use std::process::exit;

const WHITE: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

pub fn print_notice(msg: &str) {
    println!("{}{}notice{}: {}", BLUE, BOLD, WHITE, msg);
}

pub fn print_error(msg: &str) {
    eprintln!("{}{}error:{}: {}", RED, BOLD, WHITE, msg);
}

pub fn print_warn(msg: &str) {
    eprintln!("{}{}warn{}: {}", YELLOW, BOLD, WHITE, msg);
}

pub fn throw_critical_error(msg: &str) {
    print_error(msg);
    exit(1);
}