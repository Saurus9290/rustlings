#[allow(dead_code)]
mod delicious_snacks {
    // Make the imports public so they can be accessed from outside the module.
    pub use self::fruits::PEAR as fruit;      // Renaming PEAR to fruit and making it public
    pub use self::veggies::CUCUMBER as veggie; // Renaming CUCUMBER to veggie and making it public

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,   // Accessing `fruit` which is `PEAR`
        delicious_snacks::veggie,  // Accessing `veggie` which is `CUCUMBER`
    );
}
