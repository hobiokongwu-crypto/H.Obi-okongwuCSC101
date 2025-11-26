use std::fs;

fn main() {
    fs::remove_file("../Practice_1/Data.txt").expect("Couldn't remove file");
    println!("File is removed!");
}