use std::io;

// 1. An Enum representing the difficulty of a task
enum BountyTier {
    Small,      // $1 - $30
    Medium,     // $31 - $100
    HighValue,  // $100+
}

fn main() {
    println!("Enter the bounty amount in USD:");

    // 2. Getting User Input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // 3. Converting string input to a number (u32)
    let amount: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number! Defaulting to 0.");
            0
        }
    };

    // 4. Determining the Tier using basic math & if/else
    let tier = if amount <= 30 {
        BountyTier::Small
    } else if amount <= 100 {
        BountyTier::Medium
    } else {
        BountyTier::HighValue
    };

    // 5. Using pattern matching to respond
    match tier {
        BountyTier::Small => println!("Tier: Quick warm-up task! Good for learning."),
        BountyTier::Medium => println!("Tier: Nice target! Worth dedicating a weekend."),
        BountyTier::HighValue => println!("Tier: Big bounty! Might require deep research."),
    }
}