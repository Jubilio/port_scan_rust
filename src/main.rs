mod port;
mod model;
mod common_ports;

use crate::port::scan_ports;
use crate::model::Subdomain;

fn main() {
    let mut subdomain = Subdomain {
        domain: "scanme.nmap.org".to_string(),
        open_ports: Vec::new(),
    };

    subdomain = scan_ports(subdomain);

    println!("{:?}", subdomain.open_ports);
}