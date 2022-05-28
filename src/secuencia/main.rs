use std::thread;
use std::time::Duration;

fn main() {
    fn sequence(thread_number: i32) {
        for i in 1..10 {
            println!("number {} from the spawned thread {} !", i, thread_number);
            thread::sleep(Duration::from_millis(1000));
        }
    }
    let handle_thread = thread::spawn(|| {
        let thread_number = 1;
        sequence(thread_number);
    });

    let handle_thread_2 = thread::spawn(|| {
        let thread_number = 2;
        sequence(thread_number);
    });

    sequence(0);
    handle_thread.join();
    handle_thread_2.join();
}
