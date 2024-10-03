fn main() {
    let my_option: Option<()> = None;
    // Using expect instead of unwrap with a helpful error message
    if let Some(value) = my_option {
        println!("{:?}", value);
    } else {
        println!("Option is None");
    }

    // Fixing the array by adding commas between elements
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Fixing the empty vec resizing issue (remove resizing since it's unused)
    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Using std::mem::swap to correctly swap the values
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
