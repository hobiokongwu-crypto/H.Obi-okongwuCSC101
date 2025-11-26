use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    
    let mut file = OpenOptions::new().append(true).open("../Practice_1/Data - CLIVE.txt").expect("Cannot open file");
    file.write_all("\n----Hello Brothers!----".as_bytes()).expect("Write Failed!");
    file.write_all("\nThis is the extra part forgot u asked for.".as_bytes()).expect("Write Failed!");
    println!("File append success🎆😍😎╰(*°▽°*)╯");
}