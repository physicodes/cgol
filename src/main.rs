use rand::Rng;
use std::{thread, time};

const ALIVE: bool = true;
const DEAD: bool = false;
const FRAC_ALIVE: f64 = 1./4.;

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

fn modulo(a: i32, b: i32) -> usize {
    (((a % b) + b) % b) as usize
}

fn get_neighbour_inds(index: i32) -> [usize; 8] {
    // this function could probably use a macro to tidy it up
    let w = WIDTH as i32;
    let s = SIZE as i32;
    let nn = modulo(index - w, s);
    let nw = modulo(index - w - 1, s);
    let ne = modulo(index - w + 1, s);
    let ww = modulo(index - 1, s);
    let ee = modulo(index + 1, s);
    let ss = modulo(index + w, s);
    let sw = modulo(index + w - 1, s);
    let se = modulo(index + w + 1, s);
    return [nw, nn, ne, ww, ee, sw, ss, se];
}

fn get_live_neighbours(board: &[bool; SIZE]) -> [u8; SIZE] {
    let mut live_neighbours = [0u8; SIZE];
    for i in 0..SIZE {
        let mut sum: u8 = 0;
        let neighbour_inds = get_neighbour_inds(i as i32);
        for j in neighbour_inds.iter() {
            if board[*j] == ALIVE {
                sum += 1;
            }
        }
        live_neighbours[i] = sum;
    }
    return live_neighbours;
}

fn update_board(board: &mut [bool; SIZE], live_neighbours: &[u8; SIZE]) {
    for (state, neighbours) in board.iter_mut().zip(live_neighbours.iter()) {
        if *neighbours == 3 {
            *state = ALIVE;
        } else if *neighbours == 2 && *state == ALIVE {
            *state = ALIVE;
        } else {
            *state = DEAD;
        }
    }
}

fn main() {

    // Init board
    let mut board = init_board();

    // Init loop variables
    let mut n: u32 = 0;
    let wait_duration = time::Duration::new(REFRESH_RATE, 0);

    // Start loop
    loop {

        // Increment counter
        n += 1;

        // Display
        print!("{}[2J", 27 as char); // clear screen
        print_board(&board);
        println!("{}th iteration", n);

        // Update
        let live_neighbours = get_live_neighbours(&board);
        update_board(&mut board, &live_neighbours);

        // Wait
        thread::sleep(wait_duration);
    }
}
