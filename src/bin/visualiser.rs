use cgol::Board;
use std::{thread, time::Duration};

fn visualise_sim(frac_alive: f64) {
    let mut b = Board::from_probability(50, 20, frac_alive);
    loop {
        print!("\x1B[2J"); // clears terminal
        println!("{}", b);
        println!("{} cells alive.", b.sum());
        b.update();
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    visualise_sim(0.35);
}
