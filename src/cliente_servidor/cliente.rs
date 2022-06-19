use std::env;
use std::fmt::format;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn main() {

    let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap();
    println!("Conectado");
    println!("Enviando");
    for i in 1..6 {
        let mut tcp_stream = stream.try_clone().unwrap();
        let data = format!("{},2000\n", i);
        tcp_stream.write_all(data.as_bytes()).unwrap();
        let mut reader = BufReader::new(tcp_stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer);
        println!("{}",buffer);
    }
    sleep(Duration::from_secs(1));
    println!("Chau!");
}