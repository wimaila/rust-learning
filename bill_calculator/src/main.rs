use std::io;

fn main() {
    // 1. Get total bill price safely
    let total_price: f64 = loop {
        println!("Enter total price:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num, // Success: breaks the loop and saves the number
            Err(_) => println!("Invalid input! Please enter a valid number."),
        }
    };

    // 2. Get number of payers safely
    let payers: f64 = loop {
        println!("Enter number of payers:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                if num <= 0.0 {
                    println!("Number of payers must be greater than zero!");
                    continue;
                }
                break num;
            }
            Err(_) => println!("Invalid input! Please enter a valid number."),
        }
    };

    // 3. Get tip percentage safely
    let tip_percentage: f64 = loop {
        println!("Enter tip percentage (e.g. 5 for 5%):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input! Please enter a valid number."),
        }
    };

    // 4. Calculate final price per person
    let final_price = (total_price + (total_price * tip_percentage / 100.0)) / payers;

    println!("\nEach person pays: ${:.2}", final_price);
}   