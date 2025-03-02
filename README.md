# Sysinfo Udp Cliente

![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

This project is a client-server system where the client requests system information from the server. The server, running on a system, collects detailed information about the system's resources, such as CPU and disk data. The communication between the client and server occurs over the *UDP* protocol. 

**This project was developed in just a few hours as part of a training exercise over the weekend.**

## Features

- **Client**: Sends requests to the server for system data (CPU, disk, etc.).
- **Server**: Collects system resource data (CPU usage, disk space, etc.) and sends it back to the client.
- **Communication**: Uses the UDP protocol for communication between the client and server.
  
## Technologies Used

- **Rust**: The project is developed using the Rust programming language.
- **UDP**: Communication between the client and server occurs using the UDP protocol.
- **Serde**: For serializing and deserializing data in JSON format.

## Instalation

1. Clone the repository:
```
git clone https://github.com/Matheus-git/sysinfo-udp-client.git
```

2. Navigate to the project directory:
```
cd sysinfo-udp-client
```

3. Check flags:
```
cargo run -- -help
 ```

## 📝 License

This project is open-source under the MIT License.
