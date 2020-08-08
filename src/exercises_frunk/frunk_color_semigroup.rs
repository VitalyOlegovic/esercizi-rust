use frunk::Semigroup;
use core::fmt;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

pub fn lets_make_orange() {
    let red = Color::Red;
    let yellow = Color::Yellow;
    let result = red.combine(&yellow);
    println!("{:?}", result)
}
