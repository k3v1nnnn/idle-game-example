use std::env;
use std::io::Write;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn main() {

    let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap();
    println!("Conectado");

    loop {
        println!("Enviando");
        stream.write_all("Mensaje..".as_bytes()).unwrap();
        stream.write_all("\n".as_bytes()).unwrap();
        sleep(Duration::from_secs(1))
    }

}