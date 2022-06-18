use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::thread;
use std::time::Instant;

struct Payment {
    id: String,
    amount: String
}

impl Payment {
    fn new(id:String, amount:String) -> Payment {
        Payment{id, amount}
    }

    fn process(&self) {
        println!("[info] transaccion recibida {}", self.id);
    }
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

    for stream in listener.incoming() {
        println!("Cliente conectado");
        let mut reader = BufReader::new(stream.unwrap());

        thread::spawn(move || {
            loop {
                let mut buffer = String::new();
                reader.read_line(&mut buffer);
                if buffer.len() > 0 {
                    println!("Hello {}", buffer);
                } else {
                    println!("Goodbye!");
                    break;
                }
            }
        });
    }
}