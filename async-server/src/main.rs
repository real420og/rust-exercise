use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use async_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(8) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        let data = "<!DOCTYPE html>\
        <html>\
        <head>\
        <title>Example HTTP Response</title>\
        </head>\
        <body>\
        <h1>Hello, World!</h1>\
        <p>This is an example HTTP response.</p>\
        </body>\
        </html>\
        ";

        let response = format!(
            "HTTP/1.1 200 OK\
            \r\nContent-Type: text/html\
            \r\nContent-Length: {}\
            \r\n\r\n{}",
            data.len(),
            data
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
