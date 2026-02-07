use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, Result};

fn read_from_stream(stream: &mut TcpStream){
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(0) => println!("Connection closed"),
        Ok(n) => {
            let data = String::from_utf8_lossy(&buffer[..n]);
            println!("Received: {}", data);
        }
        Err(e) => eprint!("Error: {}", e),
    }
}

fn handler(mut stream: TcpStream) -> Result<()> {
    read_from_stream(&mut stream);
    
    stream.write_all(b"Hello World!")?;
    stream.flush()?;
    Ok(())
}

pub fn init_server() -> Result<()> {
    let conn_addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(conn_addr).unwrap();
    println!("Listening on {}", conn_addr);

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
