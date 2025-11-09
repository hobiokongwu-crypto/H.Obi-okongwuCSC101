//Rust program that displays a food menu for custmer purchasing
use std::io;

fn main() {
    println!("\n--------MENU--------\n");
    println!("CODE|       FOOD               |     PRICE   ");
    println!("  P | Poundo Yam/Edinkaiko Soup|  N3,200     ");
    println!("  F | Fried Rice & Chicken     |  N3,000     ");
    println!("  A | Amala & Ewedu Soup       |  N2,500     ");
    println!("  E | Eba & Egusi Soup         |  N2,000     ");
    println!("  W | White Rice & Stew        |  N2,500     ");

    println!("Enter food input:");
    let mut food_input = String::new();
    io::stdin().read_line(&mut food_input).expect("Failed to read input");
    let food_input = food_input.trim().to_uppercase();

    println!("Enter quantity");
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
    let quantity:i32= quantity_input.trim().parse().expect("Enter valid number");


    let price:i32 = match food_input.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _=> {
            println!("Invalid food type selected!");
            return;
        }
    };


    let mut total = price * quantity;

    if total> 10000 {
        let discount = (total as f64) * 0.05;
        total = (total as f64 - discount)as i32;
        println!("A 5% discount has been applied!");
    }

    println!("------------------------");
    println!("Food selected: {}", food_input);
    println!("quantity: {}", quantity);
    println!("Total amount to pay: N{}", total);
    println!("------------------------");
    println!("Thanks for ordering!");
}