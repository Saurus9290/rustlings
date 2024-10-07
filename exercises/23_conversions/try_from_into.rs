#![allow(clippy::useless_vec)]
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for the `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (r, g, b) = tuple;
        if (0..=255).contains(&r) && (0..=255).contains(&g) && (0..=255).contains(&b) {
            Ok(Color {
                red: r as u8,
                green: g as u8,
                blue: b as u8,
            })
        } else {
            Err(IntoColorError::IntConversion)
        }
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Color::try_from((arr[0], arr[1], arr[2]))
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        Color::try_from((slice[0], slice[1], slice[2]))
    }
}

fn main() {
    // Using the `try_from` function.
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since `TryFrom` is implemented for `Color`, we can use `TryInto`.
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use the `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);

    // or put the slice within round brackets and use `try_into`.
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}
