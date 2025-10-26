// Rust program of a Job application window (IG).

use std::io;

fn main()  {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string.");
    let age:u32 = input1.trim().parse().expect("Not a valid number.");

    println!("Are you experienced? (yes/no): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string.");

    let experience = input2.trim().to_lowercase();
    if experience == "yes" {
        if age >= 40 {
            println!("Your annual incentive is: N 1,560,000.00");
        } else if age >= 30 && age < 40 {
            println!("Your annual incentive is: N 1,480,000.00");
        } else if age <= 28 {
            println!("Your annual incentive is: N1,300,000.00");
        }
    } else {
        println!("Your annual incentive is: N100,000.00");
    }
}