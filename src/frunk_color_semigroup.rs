use frunk::Semigroup;
use std::string::ToString;
use core::fmt;
use std::fmt::{Formatter, Error};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Purple,
    Orange,
    Brown,
}

impl Semigroup for Color {
    fn combine(&self, other: &Self) -> Self {
        match (self, other) {
            (Color::Red, Color::Blue) => Color::Purple,
            (Color::Blue, Color::Red) => Color::Purple,
            (Color::Yellow, Color::Blue) => Color::Green,
            (Color::Blue, Color::Yellow) => Color::Green,
            (Color::Yellow, Color::Red) => Color::Orange,
            (Color::Red, Color::Yellow) => Color::Orange,
            (a, b) =>
                if a == b {
                    a.clone()
                } else {
                    Color::Brown
                }
        }
    }
}

impl fmt::Display for Color {

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

pub fn prova() {
    let red = Color::Red;
    let yellow = Color::Yellow;
    let result = red.combine(&yellow);
    println!("{}", result.to_string())
}