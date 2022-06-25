use std::collections::HashMap;
use std::mem::size_of;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum TransactionState {
    Accepted,
    Commit,
    Abort,
}

struct Stakeholder {
    log: HashMap<usize, TransactionState>,
    socket: UdpSocket
}

impl Stakeholder {
    fn new() -> Stakeholder {
        let ip = "127.0.0.1".to_string();
        let port = "11111".to_string();
        let log = HashMap::new();
        let socket = UdpSocket::bind( format!("{}:{}",ip, port)).unwrap();
        Stakeholder{log, socket}
    }

    fn parse_message(&self, buffer:&mut [u8]) -> (usize, u8) {
        let id_from = usize::from_le_bytes(buffer[1..].try_into().unwrap());
        (id_from, buffer[0])
    }

    fn build_message(&self, transaction:u8, id: usize) -> Vec<u8>{
        let mut msg = vec!(transaction);
        msg.extend_from_slice(&id.to_le_bytes());
        msg
    }

    pub fn response(&mut self) {
        const FAIL_PROBABILITY:f64 = 0.8;
        loop {
            let mut buffer = [0; size_of::<usize>() + 1];
            let (_size, from) = self.socket.recv_from(&mut buffer).unwrap();
            let (id, transaction) = self.parse_message(&mut buffer);

            match &transaction {
                b'P' => {
                    println!("[STAKEHOLDER] receive PREPARE for {}", id);
                    let process_failed = rand::thread_rng().gen_bool(FAIL_PROBABILITY);
                    let mut transaction = b'C';
                    if process_failed {
                        self.log.insert(id, TransactionState::Abort);
                        println!("[STAKEHOLDER] failed for {}", id);
                        transaction = b'A';
                    } else {
                        let sleep_time = rand::thread_rng().gen_range(10..15);
                        thread::sleep(Duration::from_secs(sleep_time));
                        self.log.insert(id, TransactionState::Accepted);
                        println!("[STAKEHOLDER] finish for {}", id);
                    }
                    let message = self.build_message(transaction, id);
                    self.socket.send_to(&message, from).unwrap();
                }
                b'C' => {
                    println!("[STAKEHOLDER] receive COMMIT for {}", id);
                    self.log.insert(id, TransactionState::Commit);
                    thread::sleep(Duration::from_millis(1000));
                    let transaction = b'C';
                    let message = self.build_message(transaction, id);
                    self.socket.send_to(&message, from).unwrap();
                }
                b'A' => {
                    println!("[STAKEHOLDER] receive ABORT for {}", id);
                    self.log.insert(id, TransactionState::Abort);
                    thread::sleep(Duration::from_millis(1000));
                    let transaction = b'A';
                    let message = self.build_message(transaction, id);
                    self.socket.send_to(&message, from).unwrap();
                }
                _ => {
                    println!("[STAKEHOLDER] receive UNKNOWN for {}", id);
                }
            }

        }
    }
}
fn main() {
    let mut stakeholder = Stakeholder::new();
    stakeholder.response();
}