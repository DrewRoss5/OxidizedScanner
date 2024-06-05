mod scanner;
use std::{num::ParseIntError, process::exit};

use clap::{arg, Command};
use scanner::PortScanner;

// parses ports from a string of ports seperated by commas, returns an error if a port is invalid
fn parse_ports(port_str: &String) -> Result<Vec<i16>, ParseIntError>{
    let mut ports: Vec<i16> = Vec::new();
    for i in port_str.split(','){
        let port: i16 = i.parse()?;
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
        if i.parse::<i8>().is_err(){
            return false
        }
    }
    true
}

fn main(){
    let args = Command::new("OxiScanner").about("A simple port scanner written in rust").arg(arg!(-a --address <ADDRESS> "The IP address to be scanned").required(true))
                                                           .arg(arg!(-p --ports <PORTS> "The ports to be scanned.").required(true)).get_matches();
    // ensure the address is valid 
    let address: &String = args.get_one::<String>("address").unwrap();
    if !validate_ipv4_addr(address){
        println!("Invalid target address");
        exit(1);
    }
    // ensure all ports are valid
    let ports: Vec<i16>;
    match parse_ports(args.get_one::<String>("ports").unwrap()){
        Ok(tmp) => {ports = tmp}
        Err(_) => {
                    println!("Invalid Port");
                    exit(1)
                }
    }
    let scan = PortScanner::new(&address, ports);
    // begin the scan
    // TODO: Make this determine the type of scan (multithreaded, SYN, etc.) to run before running
    scan.scan_single_threaded();
}