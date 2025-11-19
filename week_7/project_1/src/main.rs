// Rust program for that solves mathematical calculations through different formulaes

use std::io;

fn area_of_a_trapezium(height:f64,base1:f64,base2:f64)->f64{
    let result = height/2.0*(base1 + base2);
    return result;
}

fn area_of_a_rhombus(diagonal1:f64,diagonal2:f64)->f64{
    let result = 0.5 * diagonal1 * diagonal2;
    return result;
}

fn area_of_a_parallelogram(base:f64,altitude:f64)->f64{
    let result = base * altitude;
    return result;
}

fn area_of_a_cube(side_length:f64)->f64{
    let result = 6.0 * side_length.powi(2);
    return result;
}

fn volume_of_a_cylinder(radius:f64,height:f64)->f64{
    let pi_value:f64 = 3.14159;
    let result = pi_value * radius.powi(2) * height;
    return result;
}

fn fn main() {
    println!("Welcome to the MTH 101 mensuration and geometry calculator!!!");
    println!("\n------------------------------");
    println!("Please select a formula by enterig a number:");
    println!("1. Area of a trapezium", );
    println!("2. Area of a rhombus", );
    println!("3. Area of a parallelogram", );
    println!("4. Area of a cube", );
    println!("5. Volume of a cylinder", );
    println!("--------------------------------", );

    let mut choice_input = String::new();

    io::stdin().read_line(&mut choice_input).expect("ERR: Couldn't read selection.");
    let choice:i32 = match choice_input.trim().parse(){
        Ok(num)=> num,
        Err(_)=>{
            println!("\nERR: Invalid input. Please run the program again and enter a number.");
            return;
        }
    };

    let result:f64;

    if choice == 1{
        println!("Calculating Area of Trapezium ...");

        let mut h_input = String::new();
        println!("Enter height (h):");
        io::stdin().read_line(&mut h_input).expect("Failed to read input");
        let h:f64 = h_input.trim().parse().expect("Invalid number for height");
    }
}