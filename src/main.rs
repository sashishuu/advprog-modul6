use std::{
    fs,
    io::{prelude::*, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        serve_file("hello.html", stream);
    } else {
        serve_file("404.html", stream);
    }
}

fn serve_file(filename: &str, mut stream: TcpStream) {
    let status_line;
    let contents;

    if let Ok(content) = fs::read_to_string(filename) {
        status_line = "HTTP/1.1 200 OK";
        contents = content;
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        contents = "<html><body><h1>Oops!</h1><p>Sorry, I don’t know what you’re asking for.</p></body></html>".to_string();
    }

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
