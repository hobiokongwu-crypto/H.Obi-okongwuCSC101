use std::io::Write;

fn main() {
    
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "The Cult of Tetanus";
    let mantra = "\n3-0 is not a loss if u don't let it be😁👌";

    let mut file = std::fs::File::create("Data.txt").expect("Create Failed!");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("Write Failed!");
    file.write_all(announce.as_bytes()).expect("Write Failed!");
    file.write_all(dept.as_bytes()).expect("Write Failed!");
    file.write_all(mantra.as_bytes()).expect("Write Failed!");
    println!("\nData written to file.");
}