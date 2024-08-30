fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        
        // Create a mutable reference `y` and push 42 to it.
        let y = &mut x;
        y.push(42);
        
        // Now, create a new mutable reference `z` and push 13 to it.
        let z = &mut x;
        z.push(13);

        assert_eq!(x, [42, 13]);
    }
}
