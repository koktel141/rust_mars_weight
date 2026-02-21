use std::io::{self, Write};

fn calculate_mars_weight(earth_weight: f32) -> f32 {
    (earth_weight * 3.71) / 9.81
}

fn get_input(prompt: &str) -> f32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Eror in reading input !");

    input.trim().parse().unwrap_or(0.0) 
}

fn main() {
    println!("--- Mars Weight Calculator ---");

    let weight = get_input("Enter your weight on Earth (kg): ");
    
    let mars_weight = calculate_mars_weight(weight);

    println!("Your weight on Mars is: {:.2} kg", mars_weight);
}