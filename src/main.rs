use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;

fn handle_stream(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    loop {
        let mut buf = String::new();
        match stream.read_line(&mut buf) {
            Ok(_) => {
                println!("Reading: {}", buf.trim());
                match stream.get_ref().write(buf.as_bytes()) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Error Writing: {}", e);
                        break;
                    },
                }
            }
            Err(e) => println!("Error Reading: {}", e),
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
                thread::spawn(move || {
                    handle_stream(stream);
                });
            }
            Err(e) => { println!("Connection failed {:?}", e); }
        }
    }
}
