use rand::Rng;
use std::{thread, time::Duration};
use std::error::Error;
use csv::Writer;

#[derive(Debug)]
enum State {
    Alive,
    Dead,
}

#[derive(Debug)]
struct Board {
    width: i32,
    height: i32,
    cells: Vec<State>,
}

impl Board {

    fn init(width: i32, height: i32, frac_alive: f64) -> Board {

        let mut cells = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..(width * height) {
            let p: f64 = rng.gen();
            if p < frac_alive {
                cells.push(State::Alive)
            } else {
                cells.push(State::Dead)
            }
        }

        Board {width: width, height: height, cells: cells}
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
            match cell {
                State::Alive => output.push('#'),
                State::Dead  => output.push(' '),
            }
        }

        // Print human readable output
        println!("{}", output);
    }

    fn pos_from_ind(&self, index: i32) -> (i32, i32) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    fn ind_from_pos(&self, position: (i32, i32)) -> usize {
        (self.width * position.1 + position.0) as usize
    }

    fn verify_pos(&self, position: (i32, i32)) -> (i32, i32) {

        let mut x = position.0;
        if x == -1 {
            x = self.width - 1;
        } else if x == self.width {
            x = 0;
        }

        let mut y = position.1;
        if y == -1 {
            y = self.height - 1;
        } else if y == self.height {
            y = 0;
        }

        (x, y)
    }

    fn get_neighbours(&self, index: usize) -> [usize; 8] {
        // defo wanna tidy this up with a macro
        let (x, y) = self.pos_from_ind(index as i32);
        [
            self.ind_from_pos(self.verify_pos((x-1, y-1))),
            self.ind_from_pos(self.verify_pos((x, y-1))),
            self.ind_from_pos(self.verify_pos((x+1, y-1))),
            self.ind_from_pos(self.verify_pos((x-1, y))),
            self.ind_from_pos(self.verify_pos((x+1, y))),
            self.ind_from_pos(self.verify_pos((x-1, y+1))),
            self.ind_from_pos(self.verify_pos((x, y+1))),
            self.ind_from_pos(self.verify_pos((x+1, y+1))),
        ]

    }

    fn count_neighbours(&self, index: usize) -> i32 {
        let neighbour_indices = self.get_neighbours(index);
        let mut sum: i32 = 0;
        for i in neighbour_indices.iter() {
            match self.cells[*i] {
                State::Alive => sum += 1,
                State::Dead => (),
            }
        }
        sum
    }

    fn update(&mut self) {
        let mut new_cells: Vec<State> = Vec::new();

        // let updated_cells: Vec<State> = Vec::new();
        for (i, state) in self.cells.iter().enumerate() {
            let num_neighbours = self.count_neighbours(i);
            let new_state: State;

            if num_neighbours == 3 {
                new_state = State::Alive
            } else if num_neighbours == 2 {
                new_state = match state {
                    State::Alive => State::Alive,
                    State::Dead => State::Dead,
                };
            } else {
                new_state = State::Dead;
            }
            new_cells.push(new_state);
        }

        self.cells = new_cells;
    }

    fn sum(&self) -> i32 {
        let mut sum = 0i32;
        for state in self.cells.iter() {
            match state {
                State::Alive => sum += 1,
                State::Dead => (),
            };
        }
        sum
    }

}

fn visualise_sim(frac_alive: f64) {
    let mut b = Board::init(50, 20, frac_alive);
    loop {
        print!("\x1B[2J");
        b.print_board();
        println!("{} cells alive.", b.sum());
        b.update();
        thread::sleep(Duration::from_secs(1));
    }
}

fn run_sim(frac_alive: f64, iterations: i32) -> Vec<String> {
    let mut b = Board::init(100, 100, frac_alive);
    let mut population = Vec::new();

    for _ in 0..(iterations + 1) {
        population.push(b.sum().to_string());
        b.update();
    }

    population
}

#[test]
fn test_board() {
    let mut b = Board::init(5, 5, 1.);
    let b1 = Board::init(5, 5, 0.);

    b.print_board();

    let ind: usize = 13;
    let pos = b.pos_from_ind(ind as i32);
    assert_eq!(pos, (3, 2));
    assert_eq!(b.ind_from_pos(pos), ind);
    assert_eq!(b.verify_pos((-1, -1)), (4, 4));
    assert_eq!(b.verify_pos((2, 3)), (2, 3));
    assert_eq!(b.verify_pos((5, 5)), (0, 0));
    assert_eq!(b.get_neighbours(4), [23, 24, 20, 3, 0, 8, 9, 5]);
    assert_eq!(b.count_neighbours(4), 8);
    assert_eq!(b1.count_neighbours(4), 0);
    assert_eq!(b.sum(), 25);
    b.update();
    assert_eq!(b.sum(), 0);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("out/sim_results.dat")?;
    const REPEATS: i32 = 100;
    let fracs = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    for frac in fracs.iter() {
        for _ in 0..REPEATS {
            let results = run_sim(*frac, 50);
            wtr.write_record(results)?;
        }
    }
    wtr.flush()?;
    Ok(())
}
