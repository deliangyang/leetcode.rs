use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Write, Read};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n<p>1111</p>";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("new client!");
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
