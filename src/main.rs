use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind to port 3000");
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

    let mut buffer = [0u8; 512]; // buffer is just a data/memory for storing the bytes received 
    let _n = stream.read(&mut buffer).unwrap(); // read returns how many bytes i just read 
    let client_message = String::from_utf8_lossy(&buffer); // utf is basically for converting u8 assay to string (at a high level)
    let split_message : Vec<&str> = client_message.split("\r\n").collect() ;
    let request_line = split_message[0] ;
    let url_path : Vec<&str> = request_line.split(" ").collect() ;
    // println!("{:?}" , url_path[1]) ;
    let response ;
    if url_path[1].len() >= 2 {
        response = "HTTP/1.1 404 Not Found\r\n\r\n" ;
    }else {
        response = "HTTP/1.1 200 OK\r\n\r\n" ;
    }

    stream.write(response.as_bytes()).unwrap() ;
}
