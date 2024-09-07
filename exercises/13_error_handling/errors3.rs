use std::num::ParseIntError;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// Fix the compiler error by changing the signature and body of the `main` function.
fn main() -> Result<(), ParseIntError> {  // Changed signature to return Result
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Handle the Result returned by total_cost.
    let cost = total_cost(pretend_user_input)?;  // Using ? operator to propagate errors

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }

    Ok(())  // Return Ok to satisfy the Result<(), ParseIntError> return type
}

