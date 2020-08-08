use std::fmt::{Formatter, Error};

pub enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Purple,
    Orange,
    Brown,
}

impl core::fmt::Display for Color {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        let s = match self {
            Color::Red => String::from("Red"),
            Color::Yellow => String::from("Yellow"),
            Color::Blue => String::from("Blue"),
            Color::Green => String::from("Green"),
            Color::Purple => String::from("Purple"),
            Color::Orange => String::from("Orange"),
            Color::Brown => String::from("Brown")
        };
        write!(f, "{}", s)
    }
}

pub fn print_the_color(){
    let c = Color::Green;
    println!("{}", c);
}