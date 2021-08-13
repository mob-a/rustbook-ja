use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer=[0;1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1"){
        ("HTTP/1.1 200 OK", "src/hello.html")
    }else if buffer.starts_with(b"GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "src/hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND", "src/notfound.html")
    };
    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}\r\n\r\n{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
