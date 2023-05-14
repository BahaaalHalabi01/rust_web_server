use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream)
        })
        
    }
}
fn handle_connection(mut stream: TcpStream) {
    // A response looks something like this
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //
    // ex: HTTP/1.1 200 OK\r\n\r\n
    let route_home: (String, String) =
        (String::from("HTTP/1.1 200 OK"), String::from("index.html"));
    let route_post: (String, String) = (String::from("HTTP/1.1 200 OK"), String::from("post.html"));
    let route_404: (String, String) = (
        String::from("HTTP/1.1 404 NOT FOUND"),
        String::from("404.html"),
    );
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_post = b"GET /post HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        route_home
    } else if buffer.starts_with(get_post) {
        thread::sleep(Duration::from_secs(5));
        route_post
    } else {
        route_404
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
