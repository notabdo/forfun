use std::io::{self, Write};

fn main() {
    println!("Welcome, dear customer!");

    print!("Please enter the amount: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let amount: f64 = input.trim().parse().expect("Invalid number");

    if amount <= 0.0 {
        println!("The amount cannot be zero or negative!");
    } else {
        let discount = if amount >= 500.0 {
            amount * 0.2
        } else if amount >= 100.0 {
            amount * 0.1
        } else {
            0.0
        };

        println!("Your available discount is: {:.2}", discount);
        println!("Your total amount after discount is: {:.2}", amount - discount);
    }
}
