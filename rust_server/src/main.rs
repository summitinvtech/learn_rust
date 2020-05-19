
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {        
        let mut file = File::open("index.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);

        println!("{}",response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {println!("didn't start with get");}
    println!("Request:{}", String::from_utf8_lossy(&buffer[..]));
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}
