fn main() {
    let mut res = 42;
    let option = Some(12);

    // Use `if let` to handle the Option value.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
