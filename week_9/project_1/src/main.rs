// Rust program for a file creation application for high quality drink categories
use std::io::Write;

fn main() {
    let intro1 = "Welcome to Nigerian Brewery LtD!!!\n";
    let intro2 = "Home to Nigeria's most memorably famous beverages!!!!\n";

    let mut file = std::fs::File::create("NBL LtD.txt").expect("File Creation Failed!");
    file.write_all(intro1.as_bytes()).expect("Write failed!");
    file.write_all(intro2.as_bytes()).expect("Write failed!");
    file.write_all("\nLager            | Stout           | Non-alchoholic    ".as_bytes()).expect("Write failed!");
    file.write_all("\n33 Export        | Legend          | Maltina           ".as_bytes()).expect("Write failed!");
    file.write_all("\nDesperados       | Turbo King      | Amstel Malta      ".as_bytes()).expect("Write failed!");
    file.write_all("\nGoldberg         | Williams        | Malta Gold        ".as_bytes()).expect("Write failed!");
    file.write_all("\nGulder           |                 | Fayrouz           ".as_bytes()).expect("Write failed!");
    file.write_all("\nHeineken         |                 |                   ".as_bytes()).expect("Write failed!");
    file.write_all("\nStar             |                 |                   ".as_bytes()).expect("Write failed!");
    println!("Your data has been written to the file!");
}