mod scanner;
use std::{num::ParseIntError, process::exit, time::SystemTime};

use clap::{arg, Command};
use colored::Colorize;
use scanner::PortScanner;

// parses ports from a string of ports seperated by commas, returns an error if a port is invalid
fn parse_ports(port_str: &String) -> Result<Vec<u16>, ParseIntError>{
    let mut ports: Vec<u16> = Vec::new();
    for i in port_str.split(','){
        let port: u16 = i.parse()?;
        ports.push(port);
    }
    Ok(ports)
}

// ensures a provided string is a valid IP address, returns true if it is 
fn validate_ipv4_addr(addr: &String) -> bool{
    let octets: Vec<&str> = addr.split('.').collect();
    if octets.len() != 4{
        return false
    }
    for i in octets{
        if i.parse::<u8>().is_err(){
            return false
        }
    }
    true
}

fn main(){
    let args = Command::new("OxiScanner").about("A simple port scanner written in rust").arg(arg!(-a --address <ADDRESS> "The IP address to be scanned").required(true))
                                                           .arg(arg!(-p --ports <PORTS> "The ports to be scanned.").required(true))
                                                            .arg(arg!(-t --timeout <SECONDS> "The time it takes, in seconds, for a connection attempt to timeout (DEFAULT=5)"))
                                                            .get_matches();
    // ensure the address is valid 
    let address: &String = args.get_one::<String>("address").unwrap();
    if !validate_ipv4_addr(address){
        println!("Invalid target address");
        exit(1);
    }
    // ensure all ports are valid
    let ports: Vec<u16>;
    match parse_ports(args.get_one::<String>("ports").unwrap()){
        Ok(tmp) => {ports = tmp}
        Err(_) => {
                    println!("Invalid Port");
                    exit(1)
                }
    }
    // read the user's timeout setting (if provided)
    let timeout: u64;
    match args.get_one::<String>("timeout"){
        Some(tmp) => {
            match tmp.parse::<u64>() {
                Ok(val) => {timeout = val}
                Err(_) => {
                    println!("Invalid timeout.");
                    exit(1)
                }

            }
        }
        None => {timeout = 5}
        
    }
    let scan = PortScanner::new(&address, ports);
    // begin the scan
    // TODO: Make this determine the type of scan (multithreaded, SYN, etc.) to run before running
    println!("{}", "Starting Scan...".blue()); 
    let sys_time = SystemTime::now();
    scan.scan_single_threaded(timeout);
    let elapsed = sys_time.elapsed().unwrap();
    println!("{}", "DONE".bright_blue());
    println!("Scan Completed in: {:.2} seconds", elapsed.as_secs_f64());
}
