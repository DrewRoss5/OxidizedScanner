use std::{io, net::{Shutdown, SocketAddr, TcpStream}, str::FromStr, time::Duration};
use colored::*;

pub struct PortScanner{
    ports: Vec<u16>,
    address: String
}

impl PortScanner {
    pub fn new(address_str: &str, port_vec: Vec<u16>) -> Self{
        Self { ports: port_vec, address: address_str.to_string() }
    }

    pub fn scan_single_threaded(&self, timeout: u64){
        for i in &self.ports{
            print!("{} port {}: ", self.address, i);
            match scan_port(&self.address, *i, timeout) {
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
pub fn scan_port(address: &String, port: u16, timeout: u64) -> Result<bool, io::Error>{
    let addr = SocketAddr::from_str(format!("{}:{}", address, port).as_str()).unwrap();
    match TcpStream::connect_timeout(&addr, Duration::new(timeout, 0)) {
        Ok(stream) => {
            stream.shutdown(Shutdown::Both)?;
            Ok(true)
        }
        Err(_) => {Ok(false)}
    }
}   