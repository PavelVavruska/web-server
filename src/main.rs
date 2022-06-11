use std::net::TcpListener;

fn main() {
    // binding to a port that is higher than 1023
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}