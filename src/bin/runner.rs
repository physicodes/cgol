use cgol::Board;
use std::sync::mpsc;
use std::thread;
use csv::Writer;

// Board constants
const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

// Game constants
const ITERATIONS: u32 = 2000;
const REPEATS: u32 = 10;

fn threaded_runner(frac: f64) {
    println!("Starting {}", frac);
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

    println!("Starting...");
    for i in 1..50 {
        let frac = i as f64 / 50.0;
        threaded_runner(frac);
    }
    println!("...Done");

}
