use std::{io, net::{Shutdown, SocketAddr, TcpStream}, str::FromStr, thread, time::Duration};
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
            print_scan_result(&self.address, *i, timeout);
        }
    }

    pub fn scan_multi_threaded(&self, timeout: u64){
        let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
        for i in &self.ports{
            let tmp_port = *i;
            let tmp_addr = self.address.clone();
            threads.push(thread::spawn(move || {print_scan_result(&tmp_addr, tmp_port, timeout)}));
        }
        for i in threads{
            i.join().unwrap();
        }
    }

}

// prints the result of a scanport
fn print_scan_result(address: &String, port: u16, timeout: u64){
    match scan_port(address, port, timeout) {
        Ok(res) => {
            match res{
                true => {println!("{} on port {}: {}", address, port, "OPEN".bold().green())}
                false => {println!("{} on port {}: {}", address, port, "CLOSED".bold().red())}
            }
        }
        Err(_) => {println!("{}", "ERROR".on_red())}
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