fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];

    match read_file(file_path) {
        Ok(contents) => println!("{}", contents),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}