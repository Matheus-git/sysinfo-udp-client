mod socket;
use socket::Socket;

mod interfaces;

pub fn disks() -> Vec<interfaces::DiskInfo> {
    let socket: Socket = Socket::new();
    let request = interfaces::Request {
        info_type: interfaces::InfoType::Disks,
    };
    socket.send_to(request);
    socket.receive_from()
}

pub fn cpus() -> Vec<interfaces::CPUInfo> {
    let socket: Socket = Socket::new();
    let request = interfaces::Request {
        info_type: interfaces::InfoType::CPUs,
    };
    socket.send_to(request);
    socket.receive_from()
}

fn main() {
    println!("Cpus: {:?}", cpus());
    println!("Disks: {:?}", disks());
}