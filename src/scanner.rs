use std::{io, net::{Shutdown, TcpStream}};
use colored::*;

pub struct PortScanner{
    ports: Vec<u16>,
    address: String
}

impl PortScanner {
    pub fn new(address_str: &str, port_vec: Vec<u16>) -> Self{
        Self { ports: port_vec, address: address_str.to_string() }
    }

    pub fn scan_single_threaded(&self){
        for i in &self.ports{
            print!("{} port {}: ", self.address, i);
            match scan_port(&self.address, *i) {
                Ok(res) => {
                    match res{
                        true => {println!("{}", "OPEN".bold().green())}
                        false => {println!("{}", "CLOSED".bold().red())}
                    }
                }
                Err(_) => {println!("{}", "ERROR".on_red())}
            }
        }
    }
}

// attempts to connect to the provided address on the provided port, returns true if the port is open, otherwise, returns false
pub fn scan_port(address: &String, port: u16) -> Result<bool, io::Error>{
    match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(stream) => {
            stream.shutdown(Shutdown::Both)?;
            Ok(true)
        }
        Err(_) => {Ok(false)}
    }
}   