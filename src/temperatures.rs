use std::io;
use std::cmp::Ordering;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn my_parse_input(str: String) -> i32{
    return str.trim().parse::<i32>().unwrap();
}

fn compare_temperatures(x:&i32,y:&i32) -> Ordering {
    let difference = x.abs() - y.abs();
    if difference == 0 {return x.cmp(y).reverse();}
    return x.abs().cmp(&y.abs());
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
pub fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    let mut xs = inputs
        .split_whitespace()
        .map(|s|parse_input!(s, i32))
        .collect::<Vec<i32>>();
    xs.sort_by( compare_temperatures);
    let closest = xs.first();

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    let to_print = match closest {
        Some(t) => t,
        None => &0i32
    };
    println!("{}",to_print);
}
