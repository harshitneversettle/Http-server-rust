use std::{env, fs, path::Path};
#[allow(unused_variables)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use flate2::{Compression, write::GzEncoder};

fn main() {
    // let stri = b"harshi" ;
    // // let output_buff = Vec::new() ;
    // let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    // match encoder.write_all(stri){
    //     Ok(_) => {} ,
    //     Err(_) => {}
    // } ;
    // let s = match encoder.finish(){
    //     Ok(e) => {e} ,
    //     Err(err)=>{
    //         println!("{}" , err) ;
    //         return;
    //     }
    // } ;
    // println!("{:?}" , s) ;
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind to port 3000");
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
    // println!("{}" , client_message) ;
    let client_message_without_body = match client_message.find("\r\n\r\n") {
        Some(s) => &client_message[..s],
        None => &client_message,
    };
    let split_message: Vec<&str> = client_message_without_body.split("\r\n").collect();
    let request_line = split_message[0];
    let headers = &split_message[1..];
    let req_line_split: Vec<&str> = request_line.split(" ").collect();
    let url_path = req_line_split[1];
    let mut response = String::new();
    let mut body = String::new();
    // println!("{:?}" , req_line_split) ;
    // println!() ;
    // println!("without body : \n{}" , client_message_without_body) ;
    if req_line_split[0] == "POST" {
        let dir: Vec<String> = env::args().collect();
        let dir_path = dir[2].to_string();
        let file_path = req_line_split[1].trim_start_matches("/files/");
        let full_path = Path::new(&dir_path).join(file_path);
        // let content_len: usize = match headers[4].trim_start_matches("Content-Length: ").parse()
        // {
        //     Ok(e) => e,
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // };
        let mut content_len: usize = 0;
        for i in headers {
            if i.starts_with("Content-Length: ") {
                content_len = i.trim_start_matches("Content-Length: ").parse().unwrap();
                break;
            }
        }
        // println!("{:?}" ,  content_len) ;
        let body = match client_message.find("\r\n\r\n") {
            Some(index) => &client_message[index + 4..],
            None => "",
        };
        let content = &body[..content_len];
        let _ = fs::write(full_path, content);
        response = "HTTP/1.1 201 Created\r\n\r\n".to_string();
        stream.write(response.as_bytes()).unwrap();
        return;
    } else if url_path.starts_with("/echo") {
        let temp = url_path.trim_start_matches("/echo/");
        body.push_str(temp);
        // let body_len = body.len();
        // response = format!(
        //     "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        //     body_len, body
        // );
    } else if url_path.starts_with("/files") {
        let dir: Vec<String> = env::args().collect();
        let dir_path = dir[2].to_string();
        let path = url_path.trim_start_matches("/files/").to_string();
        let full_path = Path::new(&dir_path).join(path);
        if full_path.exists() {
            let read_file = match fs::read(&full_path) {
                Result::Ok(s) => s,
                Result::Err(e) => {
                    println!("{}", e);
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
            let read_string_len = read_string.len();
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                read_string_len, read_string
            );
        }
    } else if url_path == "/" {
        response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    } else {
        response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }
    for i in headers {
        // println!("{:?}", headers);
        if i.starts_with("User-Agent:") {
            let readed_header = i.trim_start_matches("User-Agent: ");
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                readed_header.len(),
                readed_header
            );
        } else if i.starts_with("Accept-Encoding: ") {
            // println!("{:?}", client_message);
            let valid_encodings = ["gzip"];
            let temp_str = i.trim_start_matches("Accept-Encoding: ");
            let encoding_split: Vec<&str> = temp_str.split(",").map(|x| x.trim()).collect();
            // println!("{:?}", encoding_split);
            // let content_encoding = String::new();
            let body_len = body.len();
            for i in valid_encodings {
                // println!("{:?}" , i ) ;
                if encoding_split.contains(&i.trim()) {
                    // println!("{:?}" , body) ;
                    let body_buff = body.as_bytes();
                    let mut encoding = GzEncoder::new(Vec::new(), Compression::default());
                    match encoding.write_all(&body_buff) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                    let compressed_res = match encoding.finish() {
                        Ok(vec) => vec,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                    response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Encoding: {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n",
                        i,
                        compressed_res.len(),
                    );
                    let _ = stream.write_all(response.as_bytes());
                    let _ = stream.write_all(&compressed_res);

                    // writing two times because compressed_res is a vec , abd vec doesnot follow display trait , toh usko :? use krke send krna padta 
                    // isliye 2 baar write kr diya , first => headers , fir body , beb careful , send bytes 
                    break;
                } else {
                    response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        body_len, body
                    );
                    break;
                }
            }
        }
    }
    stream.write(response.as_bytes()).unwrap();
    return;
}
