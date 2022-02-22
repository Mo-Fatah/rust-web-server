use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs, thread, time::Duration};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_stream(stream);
        });
    }
}


fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {

        let status_line = "HTTP/1.1 OK";
        let response= res_format(status_line, "hello.html");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap()

    } else {

        let status_line = "HTTP/1.1 404 NOT FOUND";
        let response = res_format(status_line, "404.html");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    }
    thread::sleep(Duration::from_secs(5));
}

fn res_format(status_line: &str, content_file: &str) -> String {
        let content = fs::read_to_string(content_file).unwrap();
        format!("{}\r\nContent-Length: {}\r\n\r\n{}"
                               ,status_line
                               ,content.len()
                               ,content)
}
