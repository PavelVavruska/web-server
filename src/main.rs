use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // binding to a port that is higher than 1023
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("<= Request: {}", String::from_utf8_lossy(&buffer[..])); // TcpStream in

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        println!("=> Response: {}", response); // TcpStream out

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // some other request
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        
        println!("=> Response: {}", response); // TcpStream out
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}