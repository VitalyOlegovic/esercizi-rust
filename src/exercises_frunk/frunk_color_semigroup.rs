use frunk::Semigroup;

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
        let red_blue_purple = vec![Color::Red,Color::Blue,Color::Purple];
        let blue_yellow_green = vec![Color::Blue,Color::Yellow,Color::Green];
        let red_yellow_orange = vec![Color::Red,Color::Yellow,Color::Orange];

        match (self, other) {
            (Color::Red, Color::Blue) => Color::Purple,
            (Color::Blue, Color::Red) => Color::Purple,
            (Color::Yellow, Color::Blue) => Color::Green,
            (Color::Blue, Color::Yellow) => Color::Green,
            (Color::Yellow, Color::Red) => Color::Orange,
            (Color::Red, Color::Yellow) => Color::Orange,
            (a, b) if a==b => a.clone(),
            (a,b) if
            vec![a,b].iter().all(
                |x| red_blue_purple.contains(x)) => Color::Purple,
            (a,b) if vec![a,b].iter().all(
                |x|blue_yellow_green.contains(x)) => Color::Green,
            (a,b) if vec![a,b].iter().all(
                |x|red_yellow_orange.contains(x) ) => Color::Orange,
            _ => Color::Brown
        }
    }
}

pub fn lets_make_orange() {
    let red = Color::Red;
    let yellow = Color::Yellow;
    let result = red.combine(&yellow);
    println!("{:?}", result)
}

#[test]
fn primo_test(){
    let red = Color::Red;
    let blue = Color::Blue;
    let purple = Color::Purple;
    let green = Color::Green;
    let brown = Color::Brown;
    assert_eq!(red,red.combine(&red));
    assert_eq!(purple,red.combine(&blue));
    assert_eq!(purple,red.combine(&purple));
    assert_eq!(brown,red.combine(&green));
}