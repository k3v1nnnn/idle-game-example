use std::borrow::Borrow;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::mem::size_of;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct TransactionId(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct StakeholderId(u32);

const TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum TransactionState {
    Wait,
    Commit,
    Abort,
}

struct Coordinator {
    ip: String,
    ports: HashMap<StakeholderId, String>,
    socket: UdpSocket,
    log: HashMap<TransactionId, TransactionState>,
    wait_response: Arc<(Mutex<Vec<Option<TransactionState>>>, Condvar)>
}

impl Coordinator {
    fn new() -> Coordinator {
        let ip = "127.0.0.1".to_string();
        let port = "12345".to_string();
        let mut ports = HashMap::new();
        ports.insert(StakeholderId(0), "11111".to_string());
        ports.insert(StakeholderId(1), "22222".to_string());
        let socket = UdpSocket::bind( format!("{}:{}",ip, port)).unwrap();
        let log = HashMap::new();
        let wait_response = Arc::new((Mutex::new(vec![None; 3]), Condvar::new()));
        let coordinator = Coordinator {ip, ports, socket, log, wait_response};
        let coordinator_clone = coordinator.clone();
        thread::spawn(move||coordinator_clone.responses());
        coordinator
    }

    fn clone(&self) -> Coordinator {

        let ip = "".to_string();
        let ports = HashMap::new();
        let socket = self.socket.try_clone().unwrap();
        let log = HashMap::new();
        let wait_response = self.wait_response.clone();
        Coordinator {ip, ports, socket, log, wait_response}
    }

    fn show_logs(&self) {
        for (key, value) in &self.log {
            let state = match value {
                TransactionState::Commit => "COMMIT",
                TransactionState::Wait => "WAIT",
                TransactionState::Abort => "ABORT",
                _ => "UNKNOWN"
            };
            println!("{}: {}", key.0, state);
        }
    }

    fn parse_message(&self, buffer:&mut [u8]) -> (usize,u8, u8) {
        let id = usize::from_le_bytes(buffer[2..].try_into().unwrap());
        (id, buffer[1], buffer[0])
    }

    fn build_message(&self, transaction:u8, t: TransactionId) -> Vec<u8>{
        let mut msg = vec!(transaction);
        msg.extend_from_slice(&t.0.to_le_bytes());
        msg
    }

    fn process(&mut self, transaction_id: TransactionId, send_to: Vec<StakeholderId>) -> bool {
        if self.prepare(transaction_id, send_to.clone()) {
            self.commit(transaction_id,send_to.clone())
        } else {
            !self.abort(transaction_id,send_to.clone())
        }
    }

    fn prepare(&mut self, transaction_id: TransactionId, send_to: Vec<StakeholderId>) -> bool {
        println!("[COORDINATOR] send PREPARE for {}", transaction_id.0);
        self.log.insert(transaction_id, TransactionState::Wait);
        self.broadcast_and_wait(b'P', transaction_id, send_to, TransactionState::Commit)
    }

    fn commit(&mut self, transaction_id: TransactionId, send_to: Vec<StakeholderId>) -> bool {
        println!("[COORDINATOR] send COMMIT for {}", transaction_id.0);
        self.log.insert(transaction_id, TransactionState::Commit);
        self.broadcast_and_wait(b'C', transaction_id, send_to, TransactionState::Commit)
    }

    fn abort(&mut self, transaction_id: TransactionId, send_to: Vec<StakeholderId>) -> bool {
        println!("[COORDINATOR] send ABORT for {}", transaction_id.0);
        self.log.insert(transaction_id, TransactionState::Abort);
        self.broadcast_and_wait(b'A', transaction_id, send_to, TransactionState::Abort)
    }

    fn broadcast_and_wait(&self, message: u8, transaction_id: TransactionId, send_to: Vec<StakeholderId>, expected_state: TransactionState) -> bool {
        *self.wait_response.0.lock().unwrap() = vec![None; send_to.len()];
        let msg = self.build_message(message, transaction_id);
        for to in send_to {
            let stakeholder_port = self.ports.get(&to).unwrap();
            println!("[COORDINATOR] send {} for transaction id {} from stakeholder id {}", message, transaction_id.0, to.0);
            self.socket.send_to(&msg, format!("{}:{}",self.ip, stakeholder_port)).unwrap();
        }
        let responses = self.wait_response.1.wait_timeout_while(
            self.wait_response.0.lock().unwrap(),
            TIMEOUT,
            |resp| resp.iter().any(Option::is_none)).unwrap();
        if responses.1.timed_out(){
            println!("[COORDINATOR] send timeout {}", transaction_id.0);
            false
        } else {
            let status = responses.0.iter().all(|resp| resp.is_some() && resp.unwrap() == expected_state);
            println!("[COORDINATOR] send status {} for {}",status, transaction_id.0 );
            status
        }
    }

    fn responses(&self){
        loop {
            let mut buf = [0; size_of::<usize>() + 2];
            let (_size, _from) = self.socket.recv_from(&mut buf).unwrap();
            let (id,id_from, transaction) = self.parse_message(&mut buf);
            match &transaction {
                b'C' => {
                    println!("[COORDINATOR] receive COMMIT for transaction id {} from stakeholder id {}", id, id_from);
                    self.wait_response.0.lock().unwrap()[id_from as usize] = Some(TransactionState::Commit);
                    self.wait_response.1.notify_all();
                }
                b'A' => {
                    println!("[COORDINATOR] receive ABORT for transaction id {} from stakeholder id {}", id, id_from);
                    self.wait_response.0.lock().unwrap()[id_from as usize] = Some(TransactionState::Abort);
                    self.wait_response.1.notify_all();
                }
                _ => {
                    println!("[COORDINATOR] receive UNKNOWN for transaction id {} from stakeholder id {}", id, id_from);
                }
            }
        }
    }
}

fn main() {

    let mut coordinator = Coordinator::new();
    let mut count = 0 ;
    loop {
        coordinator.process(TransactionId(count), vec![StakeholderId(0)]);
        thread::sleep(Duration::from_secs(10));
        if count%4==0 {
            coordinator.show_logs();
        }
        count+=1;
        println!("Deje de Dormir");
        println!();
        println!();
    }

}