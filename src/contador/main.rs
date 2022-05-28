use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let _counter = Arc::new(Mutex::new(0));
    fn counter(thread_number: i32, to_count: Arc<Mutex<i32>>) {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(2000));
            let mut _count = to_count.lock().unwrap();
            *_count += 1;
            println!("sum number from the spawned thread {} in iteration {} !", thread_number, i);
        }
    }

    let count_1 = _counter.clone();
    let handle_thread = thread::spawn(move || {
        let thread_number = 1;

        counter(thread_number, count_1);
    });

    let count_2 = _counter.clone();
    let handle_thread_2 = thread::spawn(move || {
        let thread_number = 2;
        counter(thread_number, count_2);
    });

    handle_thread.join();
    handle_thread_2.join();
    println!("total count {} !", _counter.lock().unwrap());
}
