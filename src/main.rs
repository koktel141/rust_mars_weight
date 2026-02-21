use std::io::{self, Write};
use colored::*;

fn calculate_mars_weight(earth_weight: f32) -> f32 {
    (earth_weight * 3.71) / 9.81
}

fn get_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string() 
}

fn get_input(prompt: &str) -> f32 {
    let s = get_string(prompt);
    s.parse().unwrap_or(0.0) 
}
fn main() {
    let name = get_string("Enter your name: ");
    let weight = get_input("Enter your weight: ");

    println!("Hello {}, on Mars you weigh: {} kg", name.green(), format!("{:.2}", calculate_mars_weight(weight)).yellow().bold());
    get_string("\nPress Enter to exit..."); 
}
