use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "3000";

    let end_point = HOST.to_owned() + ":" + PORT;

    let listner = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at at port {PORT}");

    for stream in listner.incoming() {
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response_content = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", response_content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
