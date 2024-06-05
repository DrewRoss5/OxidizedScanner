mod scanner;
use clap::{arg, Command};
use colored::*;
use scanner::PortScanner;

fn main(){
    print!("8.8.8.8 on port 443: ");
    match scanner::scan_port(&"8.8.8.8".to_string(), 443).unwrap() {
        true => {println!("{}", "OPEN".green())}
        false => {println!("{}", "CLOSED".red())}
    }
}