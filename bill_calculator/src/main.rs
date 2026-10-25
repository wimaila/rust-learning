use std::io;

fn main() {
    // 1. Get total bill price
    println!("Enter total price:");
    let mut total_price_input = String::new();
    io::stdin()
        .read_line(&mut total_price_input)
        .expect("Failed to read line");
    let total_price: f64 = total_price_input.trim().parse().expect("Please enter a number");

    // 2. Get number of payers
    println!("Enter number of payers:");
    let mut payers_input = String::new();
    io::stdin()
        .read_line(&mut payers_input)
        .expect("Failed to read line");
    let payers: f64 = payers_input.trim().parse().expect("Please enter a number");

    // 3. Get tip percentage
    println!("Enter tip percentage (e.g. 5 for 5%):");
    let mut tip_input = String::new();
    io::stdin()
        .read_line(&mut tip_input)
        .expect("Failed to read line");
    let tip_percentage: f64 = tip_input.trim().parse().expect("Please enter a number");

    // 4. Calculate final price per person (using f64 so decimals work correctly)
    let final_price = (total_price + (total_price * tip_percentage / 100.0)) / payers;

    println!("Each person pays: ${:.2}", final_price);
}