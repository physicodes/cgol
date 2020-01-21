use cgol::Board;
use std::sync::mpsc;
use std::thread;
use csv::Writer;

// Board constants
const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

// Game constants
const ITERATIONS_MED: u32 = 5000;
const ITERATIONS_HIGH: u32 = 20000;
const REPEATS: u32 = 100;

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
            let results = if (frac < 0.13) & (frac > 0.05) {
                board.run(ITERATIONS_HIGH)
            } else {
                board.run(ITERATIONS_MED)
            };
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
