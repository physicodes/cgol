use cgol::Board;
use std::sync::mpsc;
use std::thread;
use csv::Writer;

// Board constants
const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

// Game constants
const ITERATIONS: u32 = 200;
const REPEATS: u32 = 100;

// Frac lists
const FULL_RANGE: [f64; 9] = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
const STRANGE_RANGE: [f64; 6] = [0.10, 0.12, 0.14, 0.16, 0.18, 0.20];

fn threaded_runner(frac: f64) {
    let mut wtr = Writer::from_path(format!("analysis/data/{}.csv", frac))
        .unwrap();

    let (tx, rx) = mpsc::channel();
    for _ in 0..REPEATS {
        let tx_clone = mpsc::Sender::clone(&tx);
        let frac_clone = frac;
        thread::spawn(move || {
            let mut board = Board::from_probability(WIDTH, HEIGHT, frac_clone);
            let results = board.run(ITERATIONS);
            tx_clone.send(results).unwrap();
        });
    }

    let mut n_recieved = 0;
    for result in rx {
        n_recieved += 1;
        wtr.write_record(&result).unwrap();
        if n_recieved == REPEATS {
            break;
        }
    }

    wtr.flush().unwrap();
}

fn main() {

    println!("Starting with full range of starting fracs...");
    for frac in FULL_RANGE.iter() {
        threaded_runner(*frac);
    }
    println!("...Done");

    println!("Running with starting frac between 0.1 and 0.2...");
    for frac in STRANGE_RANGE.iter() {
        threaded_runner(*frac);
    }
    println!("...Done");

}
