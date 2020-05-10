use std::io;
use std::cmp;


macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * The while loop represents the game.
 * Each iteration represents a turn of the game
 * where you are given inputs (the heights of the mountains)
 * and where you have to print an output (the index of the mountain to fire on)
 * The inputs you are given are automatically updated according to your last actions.
 **/
fn main() {

    // game loop
    loop {
        let mut maximum: Option<(i32,usize)> = None;
        for i in 0..8 as usize {
            let mut input_line : String = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let mountain_h = parse_input!(input_line, i32); // represents the height of one mountain.
            maximum = match maximum {
                Some(m,j) => if m>mountain_h{Some(m,j)}else{Some(mountain_h,i)},
                None => Some(mountain_h,i)
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        match maximum {
            Some(m,j) => println!("{}",m)
        }
        //println!("4"); // The index of the mountain to fire on.
    }
}
