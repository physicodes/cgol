use rand::Rng;
use std::{thread, time};

const ALIVE: bool = true;
const DEAD: bool = false;
const FRAC_ALIVE: f64 = 1./10.;

const WIDTH: usize = 60;
const HEIGHT: usize = 20;
const SIZE: usize = WIDTH * HEIGHT;

const REFRESH_RATE: u64 = 1; // in seconds

fn print_board(board: &[bool; SIZE]) {

    // Initialize empty string
    let mut output = String::from(""); 

    // Loop through board contents
    for (i, cell) in board.iter().enumerate() {

        // New line on edges of board
        if i % WIDTH == 0 {
            output.push('\n');
        }

        // Hashtag for alive, space for dead
        if *cell == ALIVE {
            output.push('#');
        } else if *cell == DEAD {
            output.push(' ');
        }
    }

    println!("{}", output);
}

fn init_board() -> [bool; SIZE] {

    let mut board: [bool; SIZE] = [false; SIZE];
    let mut rng = rand::thread_rng();

    for cell in board.iter_mut() {
        let random: f64 = rng.gen();
        if random < FRAC_ALIVE {
            *cell = true;
        }
    }

    return board;
}

fn main() {
    let board = init_board();
    print_board(&board);
    let mut n: u32 = 0;
    let wait_duration = time::Duration::new(REFRESH_RATE, 0);
    loop {
        n += 1;
        println!("{}", n);
        thread::sleep(wait_duration);
    }
}
