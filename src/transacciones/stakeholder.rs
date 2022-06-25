use std::fmt::format;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;


struct Logger {}
impl Logger {
    fn log( level:&str, message:String) {
        println!("[{}] {}", level, message);
    }

    pub fn info(message:String) {
        Logger::log("INFO", message);
    }

    pub fn error(message:String) {
        Logger::log("ERROR", message);
    }
}

struct Payment {
    id: String,
    amount: String
}

impl Payment {
    fn new(payment: String) -> Payment {
        let payment_split = payment.split(",");
        let payment_vec: Vec<&str> = payment_split.collect();
        let id = String::from(payment_vec[0]);
        let amount = String::from(payment_vec[1]);
        Payment{id, amount}
    }

    fn process(&self) -> i32 {
        const FAIL_PROBABILITY:f64 = 0.1;
        let mut process_time = -1;
        Logger::info(format!("transaccion recibida {}", self.id));
        Logger::info(format!("procesando {}", self.id));
        let process_failed = rand::thread_rng().gen_bool(FAIL_PROBABILITY);
        if process_failed {
            println!("[info] transaccion fallida {}", self.id);
        } else {
            let sleep_time = rand::thread_rng().gen_range(1..10);
            thread::sleep(Duration::from_secs(sleep_time));
            process_time = sleep_time as i32;
            println!("[info] transaccion finalizada {}", self.id);
        }
        return process_time;
    }
}



fn main() {
    const IP_SERVER: &str = "127.0.0.1:12345";
    let listener = TcpListener::bind(IP_SERVER).unwrap();
    for stream in listener.incoming() {
        let tcp_stream = stream.unwrap();
        let mut writer = tcp_stream.try_clone().unwrap();
        let mut reader = BufReader::new(tcp_stream);
        let mut stream_cliente = TcpStream::connect("127.0.0.1:11111").unwrap();
        thread::spawn(move || {
            loop {
                let mut buffer = String::new();
                reader.read_line(&mut buffer);
                println!("[info] entidad nueva linea {}", buffer);
                match buffer.as_str() {
                    "kill\n" => {
                        println!("[info] entidad finalizada");
                        break;
                    },
                    "\n" => {
                        println!("[info] entidad finalizada");
                        break;
                    },
                    "" => {
                        println!("[info] entidad finalizada");
                        break;
                    },
                    _ => {
                        let payment = Payment::new(buffer);
                        let process_time = payment.process();
                        let data = format!("datos {} \n", process_time);
                        stream_cliente.write_all(data.as_bytes()).unwrap();
                    }
                }
            }
        });
    }
}