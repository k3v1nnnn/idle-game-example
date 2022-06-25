use std::{env, thread};
use std::collections::HashMap;
use std::fmt::format;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use rand::thread_rng;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Stakeholder {
    BANK
}

struct Coordinator {
    ip: String,
    port: String,
    stakeholders_ports: HashMap<Stakeholder, String>,
    stakeholders: Vec<Stakeholder>
}

impl Coordinator {
    fn new() -> Coordinator {
        let ip = "127.0.0.1".to_string();
        let port = "12345".to_string();
        let mut stakeholders_ports = HashMap::new();
        stakeholders_ports.insert(Stakeholder::BANK, ":11111".to_string());
        let stakeholders = vec![Stakeholder::BANK];
        Coordinator {ip, port, stakeholders_ports, stakeholders}
    }

    fn parse_response(buffer: String) {
        
    }

    fn responses(&mut self) -> Vec<JoinHandle<()>> {
        let mut handlers = vec![];
        for stakeholder in &self.stakeholders {
            let port = self.stakeholders_ports.get(stakeholder).unwrap();
            let address = self.ip.to_owned() + port;
            println!("{}", address);
            let handler = thread::spawn(move || {
                let listener = TcpListener::bind(address).unwrap();
                for stream in listener.incoming() {
                    let tcp_stream = stream.unwrap();
                    let mut reader = BufReader::new(tcp_stream);
                    loop {
                        let mut buffer = String::new();
                        reader.read_line(&mut buffer);
                        match buffer.as_str() {
                            "\n" => {
                                println!("[info] salto de linea");
                                break;
                            },
                            "" => {
                                println!("[info] vacio");
                                break;
                            },
                            _ => {
                                println!("{}",buffer);
                            }
                        }
                    }
                    println!("[info] saliiiiii");
                }});
            handlers.push(handler);
        }

        return handlers;
    }
}

fn main() {

    let mut coordinator = Coordinator::new();
    let responses_handler = coordinator.responses();

    let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap();
    println!("Conectado");
    println!("Enviando");
    for i in 1..6 {
        let mut tcp_stream = stream.try_clone().unwrap();
        let data = format!("{},2000\n", i);
        tcp_stream.write_all(data.as_bytes()).unwrap();
    }
    sleep(Duration::from_secs(1));
    let mut tcp_stream = stream.try_clone().unwrap();
    let data = format!("\n");
    tcp_stream.write_all(data.as_bytes()).unwrap();
    println!("Chau!");

    for response_handler in responses_handler {
        response_handler.join();
    }
}