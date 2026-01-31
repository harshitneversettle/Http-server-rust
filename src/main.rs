use std::{env, fs, path::Path};
#[allow(unused_variables)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").expect("Could not bind to port 3000");
    // println!("TCP server listening");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _handle = thread::spawn(|| {
                    println!("Client connected!");
                    handle_client(stream);
                });
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
    let n = stream.read(&mut buffer).unwrap(); // read returns how many bytes i just read 
    let client_message = String::from_utf8_lossy(&buffer[..n]); // utf is basically for converting u8 assay to string (at a high level)
    let client_message_without_body = client_message.trim_end_matches("\r\n\r\n");
    let split_message: Vec<&str> = client_message_without_body.split("\r\n").collect();
    let request_line = split_message[0];
    let headers = &split_message[1..];
    let req_line_split: Vec<&str> = request_line.split(" ").collect();
    let url_path = req_line_split[1];
    let mut response = String::new();
    // println!("{}" , url_path) ;

    match url_path {
        _ if url_path.starts_with("/echo") => {
            let body = url_path.trim_start_matches("/echo/");
            let body_len = body.len();
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                body_len, body
            );
        }
        _ if url_path.starts_with("/files") => {
            let dir: Vec<String> = env::args().collect();
            let dir_path = dir[2].to_string();
            println!("{:?}", dir);
            let path = url_path.trim_start_matches("/files/").to_string();
            let full_path = Path::new(&dir_path).join(path);
            println!("okok1");
            if full_path.exists() {
                println!("{}", full_path.display());
                let read_file = match fs::read(&full_path) {
                    Result::Ok(s) => s,
                    Result::Err(e) => {
                        println!("{} okok", e);
                        return;
                    }
                };
                let read_string = match String::from_utf8(read_file) {
                    Result::Ok(s) => s,
                    Result::Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                println!("{}", read_string);
                let read_string_len = read_string.len();
                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                    read_string_len, read_string
                );
            }
        }

        "/" => {
            response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        }

        _ => {
            response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
        }
    }
    for i in headers {
        if i.starts_with("User-Agent:") {
            let readed_header = i.trim_start_matches("User-Agent: ");
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                readed_header.len(),
                readed_header
            );
        }
    }
    stream.write(response.as_bytes()).unwrap();
}
