use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

fn main() {
    println!("Welcome to the Python Developer Job Portal!");
    println!("{}", "-".repeat(45));

    let python = read_line("Do you know Python? (yes/no): ");
    let experience_str = read_line("How many years of experience or projects do you have? ");
    let experience: u32 = experience_str.trim().parse().expect("Invalid number");
    let degree = read_line("Do you have a CS degree or completed a Bootcamp? (yes/no): ");

    if python == "yes" && (experience >= 2 || degree == "yes") {
        println!("Congratulations! You have been accepted to the next interview stage.");
    } else {
        println!("Sorry, your current qualifications do not match the job requirements.");
    }
}
