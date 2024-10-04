fn main() {
    // Use the `from` function.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // Since `From` is implemented for Person, we are able to use `Into`.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    
    // Implement the Default trait for Person
    impl Default for Person {
        fn default() -> Self {
            Self {
                name: String::from("John"),
                age: 30,
            }
        }
    }
    
    // Implement From<&str> for Person
    impl From<&str> for Person {
        fn from(s: &str) -> Self {
            let parts: Vec<&str> = s.split(',').collect();
            
            // Ensure there are exactly two parts (name and age)
            if parts.len() != 2 {
                return Person::default();
            }
    
            let name = parts[0].trim();
            let age_str = parts[1].trim();
    
            // Check if the name is empty
            if name.is_empty() {
                return Person::default();
            }
    
            // Parse age into u8, return default if it fails
            if let Ok(age) = age_str.parse::<u8>() {
                Person {
                    name: name.to_string(),
                    age,
                }
            } else {
                Person::default()
            }
        }
    }
    
    fn main() {
        // Use the `from` function.
        let p1 = Person::from("Mark,20");
        println!("{p1:?}");
    
        // Since `From` is implemented for Person, we are able to use `Into`.
        let p2: Person = "Gerald,70".into();
        println!("{p2:?}");
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_default() {
            let dp = Person::default();
            assert_eq!(dp.name, "John");
            assert_eq!(dp.age, 30);
        }
    
        #[test]
        fn test_bad_convert() {
            let p = Person::from("");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    
        #[test]
        fn test_good_convert() {
            let p = Person::from("Mark,20");
            assert_eq!(p.name, "Mark");
            assert_eq!(p.age, 20);
        }
    
        #[test]
        fn test_bad_age() {
            let p = Person::from("Mark,twenty");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    
        #[test]
        fn test_missing_comma_and_age() {
            let p: Person = Person::from("Mark");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    
        #[test]
        fn test_missing_age() {
            let p: Person = Person::from("Mark,");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    
        #[test]
        fn test_missing_name() {
            let p: Person = Person::from(",1");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    
        #[test]
        fn test_missing_name_and_age() {
            let p: Person = Person::from(",");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    
        #[test]
        fn test_missing_name_and_invalid_age() {
            let p: Person = Person::from(",one");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    
        #[test]
        fn test_trailing_comma() {
            let p: Person = Person::from("Mike,32,");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    
        #[test]
        fn test_trailing_comma_and_some_string() {
            let p: Person = Person::from("Mike,32,dog");
            assert_eq!(p.name, "John");
            assert_eq!(p.age, 30);
        }
    }
