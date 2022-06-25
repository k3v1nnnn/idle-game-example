use std::collections::HashMap;
use std::fmt::format;
use std::net::UdpSocket;
use std::mem::size_of;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct TransactionId(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Stakeholder {
    BANK
}

const TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum TransactionState {
    Wait,
    Commit,
    Abort,
}

struct Coordinator {
    ip: String,
    port: String,
    stakeholders: Vec<Stakeholder>,
    ports: HashMap<Stakeholder, String>,
    socket: UdpSocket,
    log: HashMap<TransactionId, TransactionState>,
    wait_response: Arc<(Mutex<Option<TransactionState>>, Condvar)>
}

impl Coordinator {
    fn new() -> Coordinator {
        let ip = "127.0.0.1".to_string();
        let port = "12345".to_string();
        let mut ports = HashMap::new();
        ports.insert(Stakeholder::BANK, "11111".to_string());
        let stakeholders = vec![Stakeholder::BANK];
        let socket = UdpSocket::bind( format!("{}:{}",ip, port)).unwrap();
        let log = HashMap::new();
        let wait_response = Arc::new((Mutex::new(None), Condvar::new()));
        let coordinator = Coordinator {ip, port, stakeholders, ports, socket, log, wait_response};
        let coordinator_clone = coordinator.clone();
        thread::spawn(move||coordinator_clone.responses());
        coordinator
    }

    fn clone(&self) -> Coordinator {

        let ip = "".to_string();
        let port = "".to_string();
        let mut ports = HashMap::new();
        let stakeholders = vec![Stakeholder::BANK];
        let socket = self.socket.try_clone().unwrap();
        let log = HashMap::new();
        let wait_response = self.wait_response.clone();
        Coordinator {ip, port, stakeholders, ports, socket, log, wait_response}
    }

    fn parse_message(&self, buffer:&mut [u8]) -> (usize, u8) {
        let id_from = usize::from_le_bytes(buffer[1..].try_into().unwrap());
        (id_from, buffer[0])
    }

    fn build_message(&self, transaction:u8, t: TransactionId) -> Vec<u8>{
        let mut msg = vec!(transaction);
        msg.extend_from_slice(&t.0.to_le_bytes());
        msg
    }

    fn process(&mut self, transaction_id: TransactionId, send_to: Stakeholder) -> bool {
        if self.prepare(transaction_id, send_to) {
            self.commit(transaction_id,send_to)
        } else {
            !self.abort(transaction_id,send_to)
        }
    }

    fn prepare(&mut self, transaction_id: TransactionId, send_to: Stakeholder) -> bool {
        println!("[COORDINATOR] send PREPARE for {}", transaction_id.0);
        self.log.insert(transaction_id, TransactionState::Wait);
        self.broadcast_and_wait(b'P', transaction_id, send_to, TransactionState::Commit)
    }

    fn commit(&mut self, transaction_id: TransactionId, send_to: Stakeholder) -> bool {
        println!("[COORDINATOR] send COMMIT for {}", transaction_id.0);
        self.log.insert(transaction_id, TransactionState::Commit);
        self.broadcast_and_wait(b'C', transaction_id, send_to, TransactionState::Commit)
    }

    fn abort(&mut self, transaction_id: TransactionId, send_to: Stakeholder) -> bool {
        println!("[COORDINATOR] send ABORT for {}", transaction_id.0);
        self.log.insert(transaction_id, TransactionState::Abort);
        self.broadcast_and_wait(b'A', transaction_id, send_to, TransactionState::Abort)
    }

    fn broadcast_and_wait(&self, message: u8, transaction_id: TransactionId, send_to: Stakeholder, expected_state: TransactionState) -> bool {
        *self.wait_response.0.lock().unwrap() = None;
        let mut msg = self.build_message(message, transaction_id);
        let stakeholder_port = self.ports.get(&send_to).unwrap();
        println!("[COORDINATOR] send {} for {}", message, transaction_id.0);
        self.socket.send_to(&msg, format!("{}:{}",self.ip, stakeholder_port)).unwrap();
        let response = self.wait_response.1.wait_timeout_while(
            self.wait_response.0.lock().unwrap(),
            TIMEOUT,
            |resp| resp.is_none()).unwrap();
        if response.1.timed_out(){
            println!("[COORDINATOR] send timeout {}", transaction_id.0);
            false
        } else {
            let status = response.0.is_some() && response.0.unwrap() == expected_state;
            println!("[COORDINATOR] send status {} for {}",status, transaction_id.0 );
            status
        }
    }

    fn responses(&self){
        loop {
            let mut buf = [0; size_of::<usize>() + 1];
            let (size, from) = self.socket.recv_from(&mut buf).unwrap();
            let (id, transaction) = self.parse_message(&mut buf);
            match &transaction {
                b'C' => {
                    println!("[COORDINATOR] receive COMMIT for {}", id);
                    *self.wait_response.0.lock().unwrap() = Some(TransactionState::Commit);
                    self.wait_response.1.notify_all();
                }
                b'A' => {
                    println!("[COORDINATOR] receive ABORT for {}", id);
                    *self.wait_response.0.lock().unwrap() = Some(TransactionState::Abort);
                    self.wait_response.1.notify_all();
                }
                _ => {
                    println!("[COORDINATOR] receive UNKNOWN for {}", id);
                }
            }
        }
    }
}

fn main() {

    let mut coordinator = Coordinator::new();
    let mut count = 0 ;
    loop{
        coordinator.process(TransactionId(count), Stakeholder::BANK);
        thread::sleep(Duration::from_secs(10));
        count+=1;
        println!("Deje de Dormir");
    }

}