use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;

fn handle_stream(mut stream: TcpStream) {
    let mut buf = String::new();
    
    match stream.read_to_string(&mut buf) {
        Ok(_) => {
            println!("Reading fine {}", buf);
        }
        Err(e) => {
            println!("Error Reading: {}", e);
        }
    }
    println!("Reading: {:?}",buf.trim());
    
    match stream.write(buf.as_bytes()) {
        Ok(_) => {println!("Writing: {}", buf)}
        Err(e) => {
            println!("Error Writing : {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    
    //accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection!");
                handle_stream(stream);
            }
            Err(e) => { println!("Connection failed {:?}", e); }
        }
    }
}
