use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn signa_to_direction( signa: (i32,i32) ) -> String {
    let s = match signa {
        (0,-1) => "N",
        (0,1) => "S",
        (-1,0) => "W",
        (1,0) => "E",
        (1,1) => "SE",
        (1,-1) => "NE",
        (-1,1) => "SW",
        (-1,-1) => "NW",
        _ => ""
    };
    return String::from(s);
}

fn next_move( pos_light_power: (i32,i32), pos_thor: (i32,i32) ) -> (String,(i32,i32)) {
    let difference = (pos_light_power.0 - pos_thor.0,pos_light_power.1 - pos_thor.1);
    let signa = (difference.0.signum(),difference.1.signum());
    let new_position = ( pos_thor.0+signa.0, pos_thor.1+signa.1 );
    return (signa_to_direction(signa),new_position);
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    let mut thor_position = (initial_tx,initial_ty);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.

        let result = next_move( (light_x,light_y), thor_position );
        let nm = result.0;
        thor_position = result.1;

        eprintln!("{} {}", thor_position.0, thor_position.1);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // A single line providing the move to be made: N NE E SE S SW W or NW
        println!("{}",nm);
    }
}
