use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rand::Rng;
use std::thread;
use std::time::Duration;
use wood::Wood;
mod wood;
use gold::Gold;
mod gold;
use resource::Resource;
mod resource;

fn main()
{

    const INITIAL_GOLD: f64 = 100.0;
    const INITIAL_WOOD: u32 = 0;
    const PRICE_WOOD:f64 = 50.0;
    const SLEEP_TIME:u64 = 3;

    let gold_lock = Arc::new(RwLock::new(Gold::new(INITIAL_GOLD)));
    let wood_lock = Arc::new(RwLock::new(Wood::new(INITIAL_WOOD, PRICE_WOOD)));

    let gold_generate = gold_lock.clone();
    let gold_info = gold_lock.clone();
    let gold_exchange = gold_lock.clone();
    let wood_info = wood_lock.clone();
    let wood_exchange = wood_lock.clone();

    let thread_handle_info = thread::spawn(move||loop {
        println!("==========Information=========");
        if let Ok(mut _gold) = gold_info.read() {
            println!("Gold: {} ",_gold.getAmount());
        }
        if let Ok(mut _wood) = wood_info.read() {
            println!("Wood: {} ",_wood.getAmount());
        }
        println!("==============================");
        println!();
        thread::sleep(Duration::from_secs(SLEEP_TIME));
    });

    let thread_handle_generate_gold = thread::spawn(move || loop {
        println!("Start Generate Gold");
        if let Ok(mut _gold) = gold_generate.write() {
            _gold.generate();
        }
        println!("Finish Generate Gold");
        println!();
        thread::sleep(Duration::from_secs(SLEEP_TIME));
    });

    let thread_handle_exchange = thread::spawn(move||loop {
        println!("Start Exchange");
        if let Ok(mut _gold) = gold_exchange.write() {
            if let Ok(mut _wood) = wood_exchange.write() {
                _gold.exchangeWood(_wood);
            }
        }
        println!("Finish Exchange");
        println!();
        thread::sleep(Duration::from_secs(SLEEP_TIME));
    });

    thread_handle_generate_gold.join().unwrap();
    thread_handle_info.join().unwrap();
    thread_handle_exchange.join().unwrap();
}
