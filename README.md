# OxidizedScanner
A portscanner implemented in Rust, made for educational purposes. 

# Roadmap/ToDo:
  - Add support for IPv6
  - Add support for using URLs instead of IP addresses
  - Implement multithreading

# Instalation
### Linux:
Ensure cargo is installed and run `install/install.h`. The program can be called with the `oxiscanner` command.

### Windows:
Manually run `cargo build` and run the progra with `target\debug\oxiscanner.exe` executable 

# Usage
`oxiscanner.exe -a <ADDRESS> -p <PORTS>`<br>
Where \<ADDRESS> is the IPv4 and \<PORTS> is the string of ports to be scanned seperated by commas.
## Example
A sample scan on the local host:<br>
`oxiscanner -a 127.0.0.1 -p 22,53,443,80`
This scans ports 22, 53, 443, and 80.
## Flags:
### -a, --address
Defines the IP address to be scanned

### -p, --ports
Defines the ports to be scanned

### -t, --timeout
Defines the timeout for eachport (Default is 5)
