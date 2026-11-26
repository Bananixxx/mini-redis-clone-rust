use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("Failed to read from client!");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response!");
}

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}