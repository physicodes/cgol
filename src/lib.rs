use libm::round;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Alive,
    Dead,
}

struct Cell {
    state: State,
    neighbours: [usize; 8],
}

pub struct Board {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
}

mod board_indices {

    fn pos_from_ind(index: i32, width: i32) -> (i32, i32) {
        let x = index % width;
        let y = index / width;
        (x, y)
    }

    fn ind_from_pos(position: (i32, i32), width: i32, height: i32) -> usize {
        let (x, y) = verify_pos(position, width, height);
        (width * y + x) as usize
    }

    fn verify_pos(position: (i32, i32), width: i32, height: i32) -> (i32, i32) {
        let mut x = position.0;
        if x == -1 {
            x = width - 1;
        } else if x == width {
            x = 0;
        }

        let mut y = position.1;
        if y == -1 {
            y = height - 1;
        } else if y == height {
            y = 0;
        }

        (x, y)
    }

    pub fn get_neighbours(index: i32, width: i32, height: i32) -> [usize; 8] {
        let (x, y) = pos_from_ind(index, width);
        [
            ind_from_pos((x - 1, y - 1), width, height),
            ind_from_pos((x, y - 1), width, height),
            ind_from_pos((x + 1, y - 1), width, height),
            ind_from_pos((x - 1, y), width, height),
            ind_from_pos((x + 1, y), width, height),
            ind_from_pos((x - 1, y + 1), width, height),
            ind_from_pos((x, y + 1), width, height),
            ind_from_pos((x + 1, y + 1), width, height),
        ]
    }
}

impl Board {

    pub fn from_probability(width: i32, height: i32, frac_alive: f64) -> Board {
        let mut cells = Vec::new();
        let mut rng = rand::thread_rng();
        for index in 0..(width * height) {
            let p: f64 = rng.gen();
            let state = if p < frac_alive {
                State::Alive
            } else {
                State::Dead
            };
            let neighbours = board_indices::get_neighbours(index, width, height);
            cells.push(Cell { state: state, neighbours: neighbours });
        }

        Board {
            width: width,
            height: height,
            cells: cells,
        }
    }

    pub fn from_fraction(width: i32, height: i32, frac_alive: f64) -> Board {
        // generate right number of states
        let size: i32 = width * height;
        let nr_alive: i32 = round(frac_alive * (size as f64)) as i32;
        let nr_dead: i32 = size - nr_alive;
        let mut states: Vec<State> = Vec::with_capacity(size as usize);
        states.extend(vec![State::Alive; nr_alive as usize]);
        states.extend(vec![State::Dead; nr_dead as usize]);

        // shuffle states
        let mut rng = thread_rng();
        states.shuffle(&mut rng);

        // convert cells to states
        let mut cells: Vec<Cell> = Vec::with_capacity(size as usize);
        for (index, state) in states.iter().enumerate() {
            let neighbours = board_indices::get_neighbours(index as i32, width, height);
            cells.push(Cell { state: *state, neighbours: neighbours } );
        }

        Board {
            width: width,
            height: height,
            cells: cells,
        }
    }

    pub fn update(&mut self) {
        // start vector of new states
        let mut neighbour_count = Vec::new();
        // for cell in cells
        for cell in self.cells.iter() {
        //     sum cells
            let mut sum = 0;
            for neighbour_ind in cell.neighbours.iter() {
                let neighbour_state = self.cells[*neighbour_ind].state;
                match neighbour_state {
                    State::Alive => sum += 1,
                    State::Dead => sum += 0,
                };
            }
            neighbour_count.push(sum);
        }
        // for nbr_count in counts
        for (cell, nr_neighbours) in self.cells.iter_mut().zip(neighbour_count.iter()) {
            cell.state = match nr_neighbours {
                3 => State::Alive,
                2 => match cell.state {
                    State::Alive => State::Alive,
                    State::Dead => State::Dead,
                },
                _ => State::Dead,
            };
        }
    }

    pub fn run(&mut self, iterations: u32) -> Vec<String> {
        let mut sum_list = Vec::new();

        for _ in 0..(iterations + 1) {
            sum_list.push(self.sum().to_string());
            self.update();
        }

        sum_list
    }

    pub fn sum(&self) -> i32 {
        let mut sum = 0i32;
        for cell in self.cells.iter() {
            match cell.state {
                State::Alive => sum += 1,
                State::Dead => (),
            };
        }
        sum
    }
}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Draw top of board frame
        write!(f, "+").unwrap();
        for _ in 0..self.width {
            write!(f, "-").unwrap();
        }
        write!(f, "+\n").unwrap();
        // Draw cells
        let x_lim = self.width - 1;
        for (i, cell) in self.cells.iter().enumerate() {
            let x_pos = (i as i32) % self.width;
            // Draw leading frame edge
            if x_pos == 0 {
                write!(f, "|").unwrap();
            }
            // Hashtag for alive, space for dead
            match cell.state {
                State::Alive => {
                    write!(f, "#").unwrap();
                },
                State::Dead => {
                    write!(f, " ").unwrap();
                }
            }
            // Draw trailing frame edge
            if x_pos == x_lim {
                write!(f, "|\n").unwrap();
            }
        }
        // Draw bottom of board frame
        write!(f, "+").unwrap();
        for _ in 0..self.width {
            write!(f, "-").unwrap();
        }
        write!(f, "+")
    }
}

// #[cfg(test)]
// mod tests {
// 
//     use crate::{Board, State};
// 
//     #[test]
//     fn from_probability() {
//         let b1 = Board::from_probability(5, 5, 1.);
//         assert_eq!(
//             b1,
//             Board {
//                 width: 5,
//                 height: 5,
//                 cells: vec!(State::Alive; 25)
//             }
//         );
//         let b2 = Board::from_probability(50, 3, 0.);
//         assert_eq!(
//             b2,
//             Board {
//                 width: 50,
//                 height: 3,
//                 cells: vec!(State::Dead; 150)
//             }
//         );
//     }
// 
// //    #[test]
// //    fn from_fraction() {
// //        let b1 = Board::from_fraction(2, 5, 0.25);
// //        assert_eq!(b1.sum(), 3);
// //        let b2 = Board::from_fraction(2, 5, 0.24);
// //        assert_eq!(b2.sum(), 2);
// //    }
// 
//     #[test]
//     fn print_board() {
//         let b = Board::from_probability(5, 5, 0.5);
//         println!("{}", b);
//     }
// 
//     #[test]
//     fn sum() {
//         let b1 = Board::from_probability(5, 5, 0.);
//         assert_eq!(b1.sum(), 0);
// 
//         let cells = vec![
//             State::Alive,
//             State::Alive,
//             State::Alive,
//             State::Alive,
//             State::Alive,
//             State::Dead,
//             State::Dead,
//             State::Dead,
//             State::Dead,
//         ];
//         let b2 = Board {
//             width: 3,
//             height: 3,
//             cells: cells,
//         };
//         assert_eq!(b2.sum(), 5);
// 
//         let b3 = Board::from_probability(5, 5, 1.);
//         assert_eq!(b3.sum(), 25);
//     }
// 
//     #[test]
//     fn update() {
//         // Check an instant death situation
//         let mut b1 = Board::from_probability(5, 5, 1.);
//         assert_eq!(b1.sum(), 25);
//         b1.update();
//         assert_eq!(b1.sum(), 0);
//     }
// }
