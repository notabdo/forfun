use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let pin: u32 = 1234;
    let mut balance: f64 = 5000.0;

    let pin_input: u32 = read_line("Please enter your 4-digit PIN: ")
        .parse()
        .expect("Invalid PIN");

    if pin_input == pin {
        let choice = read_line("would you like to withdraw or access your balance? ").to_lowercase();

        if choice == "access balance" {
            println!("Your current balance is: ${}", balance);
        } else if choice == "withdraw" {
            let amount: f64 = read_line("Enter the amount you want to withdraw: ")
                .parse()
                .expect("Invalid amount");

            if amount > 0.0 {
                if amount <= balance {
                    balance -= amount;
                    println!("Withdrawal successful ${}! Your new balance is: ${}", amount, balance);
                } else {
                    println!("Sorry, your balance is insufficient.");
                }
            } else {
                println!("Invalid amount.");
            }
        } else {
            println!("Invalid option. Please choose 'withdraw' or 'access balance'.");
        }
    } else {
        println!("Incorrect PIN. Please try again.");
    }
}
