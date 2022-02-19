use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let content = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 OK\r\nContent-Length: {}\r\n\r\n{}"
                           ,content.len()
                           ,content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}
