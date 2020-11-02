use std::io::prelude::*;
use std::net::{TcpStream,TcpListener};
use std::fs;
use threadpool::ThreadPool;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(e) => panic!("{}",e)
    };

    // Threads
    let pool = ThreadPool::new(4);

    println!("Listening at http://127.0.0.1:7878");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = match buffer.starts_with(get) {
        true => ("HTTP/1.1 200 OK\r\n\r\n", "index.html"),
        false => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
