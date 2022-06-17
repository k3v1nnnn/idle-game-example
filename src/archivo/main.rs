use std::fs::{File};
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/archivo/data.csv").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line_split = line.split(",");
        let vec: Vec<&str> = line_split.collect();
        println!("data: {} is empty: {}", vec[0],vec[0].is_empty());
        println!("data: {} is empty: {}", vec[1],vec[1].is_empty());
        println!("data: {} is empty: {}", vec[2],vec[2].is_empty());
        println!("data: {} is empty: {}", vec[3],vec[3].is_empty());
    }
}