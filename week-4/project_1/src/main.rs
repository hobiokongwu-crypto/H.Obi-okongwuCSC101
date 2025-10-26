// Rust program for fiding the roots of a quadratic equation.

use std::io;

fn main()  {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter for value 'A':");
    io::stdin().read_line(&mut input1).expect("Not a valid string.");
    let a:f32 = input1.trim().parse().expect("Not a valid number.");

    println!("Enter for value 'B':");
     io::stdin().read_line(&mut input2).expect("Not a valid string.");
    let b:f32 = input2.trim().parse().expect("Not a valid number.");

    println!("Enter for value 'C':");
    io::stdin().read_line(&mut input3).expect("Not a valid string.");
    let c:f32 = input3.trim().parse().expect("Not a valid number.");

    let d:f32 = -4.0 * a * c + b.powf(2.0);
    if d > 0.0 {
        let sqrt_d:f32 = d.sqrt();
        let denominator:f32 = 2.0 * a;
        let x1:f32 = (-b + sqrt_d) / denominator;
        let x2:f32 = (-b - sqrt_d) / denominator;
        println!("Nature of roots is:");
        println!("2 Distinct roots and they are : {:.2} and {:.2}", x1, x2);
    } else if d < 0.0 {
         let sqrt_d:f32 = d.sqrt();
        let denominator:f32 = 2.0 * a;
        let x1:f32 = (-b + sqrt_d) / denominator;
        let x2:f32 = (-b - sqrt_d) / denominator;
        println!("Nature of roots is:", );
        println!("Complex / Imaginary roots and they are: {:.2} and {:.2}", x1, x2);
    } else if d == 0.0 {
        let denominator:f32 = 2.0 * a;
        let x:f32 = -b / denominator;
        println!("Nature of roots is:");
         println!("Equal roots and they are: {:.2}", x);
    }
}