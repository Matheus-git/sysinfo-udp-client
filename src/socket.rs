use std::net::{IpAddr, UdpSocket, SocketAddr};
use clap::Parser;
use serde_json;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short = 'p', long = "port", value_parser, default_value = "0", help = "Port number")]
    port: u16,

    #[clap(short = 'i', long = "ip", value_parser, default_value = "127.0.0.1", help = "IP address")]
    ip: String,

    #[clap(short = 's', long = "server-port", value_parser, default_value = "0", help = "Server port number")]
    server_port: u16,

    #[clap(short = 'd', long = "server-ip", value_parser, default_value = "127.0.0.1", help = "Server IP address")]
    server_ip: String,
}

#[derive(Debug)]
pub struct Socket {
    pub udp_socket: UdpSocket,
    server_addr: SocketAddr
}

impl Socket {
    pub fn new() -> Self {
        let args: Cli = Cli::parse();
        let client_ip: IpAddr = args.ip.parse().expect("Failed to parse IP address");
        let udp_socket: UdpSocket = UdpSocket::bind(SocketAddr::new(client_ip, args.port)).expect("couldn't bind to address");

        let server_ip: IpAddr = args.server_ip.parse().expect("Failed to parse IP address");

        Socket {
            udp_socket,
            server_addr: SocketAddr::new(server_ip, args.server_port)
        }
    }

    pub fn receive_from<T>(&self) -> T
    where
        T: DeserializeOwned,  
    {
        let mut buf: [u8; 60_000] = [0; 60_000];

        let (number_of_bytes, _) = self.udp_socket.recv_from(&mut buf)
            .expect("Didn't receive data");

        serde_json::from_slice(&buf[..number_of_bytes])
            .expect("Failed to deserialize the request")
    }

    pub fn send_to<T>(&self, request: T) 
    where
        T: Serialize,  
    {
        let serialized_request: Vec<u8> = serde_json::to_vec(&request).expect("Failed to serialize request");
        
        self.udp_socket.send_to(&serialized_request, self.server_addr)
            .expect("Couldn't send data");
    }
}