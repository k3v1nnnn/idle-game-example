use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {

    const INITIAL_GOLD: f64 = 100.0;
    const INITIAL_WOOD: u32 = 0;
    const PRICE_WOOD:f64= 10.0;
    //const WOOD_KEY:String = String::from("wood");;
    //let resources = vec![WOOD_KEY];
    //let mut resources_prices = HashMap::new();
    //prices.insert(WOOD_KEY, PRICE_WOOD);
    let gold = Arc::new(RwLock::new(INITIAL_GOLD));
    let wood = Arc::new(RwLock::new(INITIAL_WOOD));
    let gold_clone = gold.clone();
    let thread_generate_gold = thread::spawn(move || loop {
        //println!("Start Generate");
        thread::sleep(Duration::from_secs(5));
        if let Ok(mut _gold) = gold_clone.write() {
            let percentage :f64 = rand::thread_rng().gen();
            let gold_old = *_gold;
            *_gold = gold_old + (percentage * gold_old);
            //println!("Generate Gold: {} ",*_gold);
        }
        //println!("Finish Generate");
        //println!();
    });
    let gold_clone2 = gold.clone();
    let wood_clone = wood.clone();
    let thread_informate = thread::spawn(move||loop {
        println!("==========Information=========");
        thread::sleep(Duration::from_secs(1));
        if let Ok(mut _gold) = gold_clone2.read() {
            println!("Gold: {} ",*_gold);
        }
        if let Ok(mut _wood) = wood_clone.read() {
            println!("Wood: {} ",*_wood);
        }
        println!("==============================");
        println!();
    });
    let gold_clone3 = gold.clone();
    let wood_clone2 = wood.clone();
    let thread_exchange = thread::spawn(move||loop {
        //println!("Start Exchange");
        thread::sleep(Duration::from_secs(1));
        if let Ok(mut _gold) = gold_clone3.write() {
            if let Ok(mut _wood) = wood_clone2.write() {
                if *_gold > PRICE_WOOD {
                    *_gold = *_gold - PRICE_WOOD;
                    *_wood = *_wood + 1;
                }
            }
        }
        //println!("Finish Exchange");
        //println!();
    });
    thread_generate_gold.join().unwrap();
    thread_informate.join().unwrap();
    thread_exchange.join().unwrap();
}
