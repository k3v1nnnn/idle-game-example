use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std_semaphore::Semaphore;

fn main() {
    const SLEEP_TIME:u64 = 1;
    let semaphore = Arc::new(Semaphore::new(1));
    let semaphore_full = Arc::new(Semaphore::new(0));
    let semaphore_empty = Arc::new(Semaphore::new(5));
    let a_buffer = Arc::new(Mutex::new(0));


    let semaphore_producer = semaphore.clone();
    let semaphore_full_producer = semaphore_full.clone();
    let semaphore_empty_producer = semaphore_empty.clone();
    let mut a_buffer_producer = a_buffer.clone();
    let producer_handle = thread::spawn(move||loop {
        semaphore_empty_producer.acquire();
        semaphore_producer.acquire();
        println!("==========Producer=========");
        thread::sleep(Duration::from_secs(SLEEP_TIME));
        let mut count = a_buffer_producer.lock().unwrap();
        *count = *count + 1;
        println!(" Count {} ", *count);
        println!("===========================");
        semaphore_producer.release();
        semaphore_full_producer.release();
    });

    let semaphore_consumer = semaphore.clone();
    let semaphore_full_consumer = semaphore_full.clone();
    let semaphore_empty_consumer = semaphore_empty.clone();
    let mut a_buffer_consumer = a_buffer.clone();
    let consumer_handle = thread::spawn(move || loop {
        semaphore_full_consumer.acquire();
        semaphore_consumer.acquire();
        println!("==========Consumer=========");
        thread::sleep(Duration::from_secs(SLEEP_TIME));
        let mut count = a_buffer_consumer.lock().unwrap();
        *count = *count - 1;
        println!(" Count {} ", *count);
        println!("===========================");
        semaphore_consumer.release();
        semaphore_empty_consumer.release();
    });

    producer_handle.join();
    consumer_handle.join();
}