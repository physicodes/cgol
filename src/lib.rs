use libm::round;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Alive,
    Dead,
}

#[derive(Debug, PartialEq)]
pub struct Board {
    width: i32,
    height: i32,
    cells: Vec<State>,
}

impl Board {

    pub fn from_probability(width: i32, height: i32, frac_alive: f64) -> Board {
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
        let mut cells: Vec<State> = Vec::with_capacity(size as usize);
        cells.extend(vec![State::Alive; nr_alive as usize]);
        cells.extend(vec![State::Dead; nr_dead as usize]);

        // shuffle states
        let mut rng = thread_rng();
        cells.shuffle(&mut rng);

        Board {
            width: width,
            height: height,
            cells: cells,
        }
    }

    fn pos_from_ind(&self, index: i32) -> (i32, i32) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    fn ind_from_pos(&self, position: (i32, i32)) -> usize {
        let (x, y) = self.verify_pos(position);
        (self.width * y + x) as usize
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
        let (x, y) = self.pos_from_ind(index as i32);
        [
            self.ind_from_pos((x - 1, y - 1)),
            self.ind_from_pos((x, y - 1)),
            self.ind_from_pos((x + 1, y - 1)),
            self.ind_from_pos((x - 1, y)),
            self.ind_from_pos((x + 1, y)),
            self.ind_from_pos((x - 1, y + 1)),
            self.ind_from_pos((x, y + 1)),
            self.ind_from_pos((x + 1, y + 1)),
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

    pub fn update(&mut self) {
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
        for state in self.cells.iter() {
            match state {
                State::Alive => sum += 1,
                State::Dead => (),
            };
        }
        sum
    }
}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Loop through board contents
        for (i, cell) in self.cells.iter().enumerate() {
            // New line on edges of board
            if i == 0 {
                continue;
            } else if (i as i32) % self.width == 0 {
                write!(f, "\n").unwrap();
            }
            // Hashtag for alive, space for dead
            match cell {
                State::Alive => {
                    write!(f, "#").unwrap();
                },
                State::Dead => {
                    write!(f, " ").unwrap();
                }
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {

    use crate::{Board, State};

    #[test]
    fn from_probability() {
        let b1 = Board::from_probability(5, 5, 1.);
        assert_eq!(
            b1,
            Board {
                width: 5,
                height: 5,
                cells: vec!(State::Alive; 25)
            }
        );
        let b2 = Board::from_probability(50, 3, 0.);
        assert_eq!(
            b2,
            Board {
                width: 50,
                height: 3,
                cells: vec!(State::Dead; 150)
            }
        );
    }

    #[test]
    fn from_fraction() {
        let b1 = Board::from_fraction(2, 5, 0.25);
        assert_eq!(b1.sum(), 3);
        let b2 = Board::from_fraction(2, 5, 0.24);
        assert_eq!(b2.sum(), 2);
    }

    #[test]
    fn print_board() {
        let b = Board::from_probability(5, 5, 0.5);
        println!("{}", b);
    }

    #[test]
    fn ind_from_pos() {
        let b = Board::from_probability(5, 5, 0.5);
        let ind: i32 = 13;
        assert_eq!(b.pos_from_ind(ind), (3, 2));
    }

    #[test]
    fn verify_pos() {
        let b = Board::from_probability(5, 5, 0.5);
        assert_eq!(b.verify_pos((-1, -1)), (4, 4));
        assert_eq!(b.verify_pos((2, 3)), (2, 3));
        assert_eq!(b.verify_pos((5, 5)), (0, 0));
    }

    #[test]
    fn get_neighbours() {
        let b = Board::from_probability(5, 5, 0.5);
        assert_eq!(b.get_neighbours(4), [23, 24, 20, 3, 0, 8, 9, 5]);
    }

    #[test]
    fn count_neighbours() {
        let b1 = Board::from_probability(5, 5, 1.);
        let b2 = Board::from_probability(5, 5, 0.);
        assert_eq!(b1.count_neighbours(4), 8);
        assert_eq!(b2.count_neighbours(4), 0);
    }

    #[test]
    fn sum() {
        let b1 = Board::from_probability(5, 5, 0.);
        assert_eq!(b1.sum(), 0);

        let cells = vec![
            State::Alive,
            State::Alive,
            State::Alive,
            State::Alive,
            State::Alive,
            State::Dead,
            State::Dead,
            State::Dead,
            State::Dead,
        ];
        let b2 = Board {
            width: 3,
            height: 3,
            cells: cells,
        };
        assert_eq!(b2.sum(), 5);

        let b3 = Board::from_probability(5, 5, 1.);
        assert_eq!(b3.sum(), 25);
    }

    #[test]
    fn update() {
        // Check an instant death situation
        let mut b1 = Board::from_probability(5, 5, 1.);
        assert_eq!(b1.sum(), 25);
        b1.update();
        assert_eq!(b1.sum(), 0);
    }
}
