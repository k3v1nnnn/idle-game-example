use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("src/archivo/data.txt").expect("create failed");
    file.write_all("Hello World2".as_bytes()).expect("write failed");
    file.write_all("\nTutorialsPoint2".as_bytes()).expect("write failed");
    println!("data written to file" );
    let mut file = OpenOptions::new().append(true).open("src/archivo/data2.txt");
    if file.is_ok(){
        println!("data written to file2" );
    }
}