use rand::Rng;
// use std::{thread, time};
// 
// const FRAC_ALIVE: f64 = 1./4.;
// 
// const WIDTH: usize = 60;
// const HEIGHT: usize = 20;
// const SIZE: usize = WIDTH * HEIGHT;

#[derive(Debug)]
enum State {
    Alive,
    Dead,
}

#[derive(Debug)]
struct Cell {
    state: State,
}

#[derive(Debug)]
struct Board {
    width: i32,
    height: i32,
    size: i32,
    cells: Vec<Cell>,
}

impl Board {

    fn init(width: i32, height: i32, frac_alive: f64) -> Board {

        let size = width * height;

        let mut cells = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            let p: f64 = rng.gen();
            if p < frac_alive {
                cells.push(Cell {state: State::Alive})
            } else {
                cells.push(Cell {state: State::Dead})
            }
        }

        Board {width: width, height: height, size: size, cells: cells}
    }

    fn print_board(&self) {

        // Initialize empty string
        let mut output = String::from(""); 

        // Loop through board contents
        for (i, cell) in self.cells.iter().enumerate() {

            // New line on edges of board
            if (i as i32) % self.width == 0 {
                output.push('\n');
            }

            // Hashtag for alive, space for dead
            match cell.state {
                State::Alive => output.push('#'),
                State::Dead  => output.push(' '),
            }
        }

        // Print human readable output
        println!("{}", output);
    }

    fn count_neighbours(&self, index: i32) {

        // get neighbour indices
        let nw = self.validate_index(index - self.width - 1);
        let nn = self.validate_index(index - self.width);
        let ne = self.validate_index(index - self.width + 1);
        let ww = self.validate_index(index - 1);
        let ee = self.validate_index(index + 1);
        let sw = self.validate_index(index + self.width - 1);
        let ss = self.validate_index(index + self.width);
        let se = self.validate_index(index + self.width + 1);

        // count living neighbours
        let mut sum = 0i32;
        for i in [nw, nn, ne, ww, ee, sw, ss, se].iter() {
            match self.cells[*i].state {
                State::Alive => sum += 1,
                State::Dead  => (),
            }
        }
        let n = 5i32;
        println!("{}", n);
    }

    fn validate_index(&self, mut index: i32) -> usize {
        // Check for wrapping at top and bottom
        println!("{}", index);
        if (index + 1) < 0 {
            index += self.size;
        } else if (index - 1) > self.size {
            index -= self.size;
        }

        // Check for wrapping at edges
        if ((index + 1) % self.width) == 0 {
            index += self.width;
        } else if ((index - 1) % self.width) == 0 {
            index -= self.width;
        }

        println!("{}", index);
        return index as usize;
    }

}

fn main() {
    let b = Board::init(3, 3, 1.);
    b.print_board();
    b.count_neighbours(0i32);
}

// 
//     fn next_state(&mut self) {
//         if self.neighbours == 3 {
//             self.state = State::Alive;
//         } else if self.neighbours == 2 && self.state == State::Alive {
//             self.state = State::Alive;
//         } else {
//             self.state = State::Dead;
//         }
//     }
// }
// 
// #[derive(Debug)]
// struct Board {
//     width: i32,
//     height: i32,
//     size: usize,
//     frac_alive: f64,
//     cells: [Cell; SIZE],
// }
// 
// impl Board {
//     fn init(width: i32, height: i32, frac_alive: f64) -> Board {
//         size = (width * height) as usize;
//         cells = [Cell {State::Dead, 0i8}; size];
//         Board {width, height, size, frac_alive, cells}
//     }
// }
// 
// fn init_board() -> [bool; SIZE] {
// 
//     let mut board: [bool; SIZE] = [false; SIZE];
//     let mut rng = rand::thread_rng();
// 
//     for cell in board.iter_mut() {
//         let random: f64 = rng.gen();
//         if random < FRAC_ALIVE {
//             *cell = true;
//         }
//     }
// 
//     return board;
// }
// 
// fn modulo(a: i32, b: i32) -> usize {
//     (((a % b) + b) % b) as usize
// }
// 
// fn get_neighbour_inds(index: i32) -> [usize; 8] {
//     // this function could probably use a macro to tidy it up
//     let w = WIDTH as i32;
//     let s = SIZE as i32;
//     let nn = modulo(index - w, s);
//     let nw = modulo(index - w - 1, s);
//     let ne = modulo(index - w + 1, s);
//     let ww = modulo(index - 1, s);
//     let ee = modulo(index + 1, s);
//     let ss = modulo(index + w, s);
//     let sw = modulo(index + w - 1, s);
//     let se = modulo(index + w + 1, s);
//     return [nw, nn, ne, ww, ee, sw, ss, se];
// }
// 
// fn update_neighbours(board: &[bool; SIZE], neighbours: &mut [u8; SIZE]) {
//     for i in 0..SIZE {
//         let mut sum: u8 = 0;
//         let neighbour_inds = get_neighbour_inds(i as i32);
//         for j in neighbour_inds.iter() {
//             if board[*j] == ALIVE {
//                 sum += 1;
//             }
//         }
//         neighbours[i] = sum;
//     }
// }
// 
// fn update_board(board: &mut [bool; SIZE], neighbours: &[u8; SIZE]) {
//     for (state, nr_neighbours) in board.iter_mut().zip(neighbours.iter()) {
//         if *nr_neighbours == 3 {
//             *state = ALIVE;
//         } else if *nr_neighbours == 2 && *state == ALIVE {
//             *state = ALIVE;
//         } else {
//             *state = DEAD;
//         }
//     }
// }
// 
// fn main() {
// 
//     // Init board and neighbours
//     let mut board = init_board();
//     let mut neighbours = [0u8; SIZE];
// 
//     // Init loop variables
//     let mut n: u32 = 0;
//     let wait_duration = time::Duration::new(REFRESH_RATE, 0);
// 
//     // Start loop
//     loop {
// 
//         // Increment counter
//         n += 1;
// 
//         // Display
//         print!("{}[2J", 27 as char); // clear screen
//         print_board(&board);
//         println!("{}th iteration", n);
// 
//         // Update
//         update_neighbours(&board, &mut neighbours);
//         update_board(&mut board, &neighbours);
// 
//         // Wait
//         thread::sleep(wait_duration);
//     }
// }
