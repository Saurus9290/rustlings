fn trim_me(input: &str) -> &str {
    // Remove whitespace from both ends of a string.
    input.trim()
}

fn compose_me(input: &str) -> String {
    // Add " world!" to the string.
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons".
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
    let trimmed = trim_me("   Hello, Rust!   ");
    println!("Trimmed: '{}'", trimmed);

    let composed = compose_me("Hello");
    println!("Composed: '{}'", composed);

    let replaced = replace_me("I think cars are cool");
    println!("Replaced: '{}'", replaced);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(
