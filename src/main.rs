use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::io::{Result};

fn handler(mut stream: TcpStream) -> Result<()> {
    stream.write_all(b"Hello World!")?;
    stream.flush()?;
    Ok(())
}

fn init_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = handler(stream);
                println!("Connection handled!");
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn main() {
    // Need to handle Result<()> for this 
    let _ = init_server();
}
