use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2000").expect("Could not bind to port 3000");
    println!("TCP server listening");

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("Client connected!");
                handle_client(stream.unwrap());
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // loop => without loop , i can only send 1 message from client to server , breaks on pressing space , with loop i can keep sending messages
    loop {
        let mut buffer = [0u8; 512]; // buffer is just a data/memory for storing the bytes received 
        let n = stream.read(&mut buffer).unwrap(); // read returns how many bytes i just read 
        let client_message = String::from_utf8_lossy(&buffer[..n]); // utf is basically for converting u8 assay to string (at a high level)
        let send = stream.write(b"fuck").unwrap();
        println!("{:?}", client_message);
    }
}
