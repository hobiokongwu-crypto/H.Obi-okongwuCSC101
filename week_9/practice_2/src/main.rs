use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("../Practice_1/Data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}