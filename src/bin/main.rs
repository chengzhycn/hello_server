use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::atomic::{AtomicUsize, Ordering};

use hello_server::ThreadPool;

static REQUESTS_CNT: AtomicUsize = AtomicUsize::new(0);

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!(
        "current requests number is {}!!!",
        REQUESTS_CNT.fetch_add(1, Ordering::SeqCst) + 1
    );
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
