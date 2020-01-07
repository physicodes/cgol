use cgol::Board;
use csv::Writer;
use std::error::Error;

fn run_game(frac_alive: f64, iterations: i32) -> Vec<String> {
    let mut b = Board::from_probability(100, 100, frac_alive);
    let mut population = Vec::new();

    for _ in 0..(iterations + 1) {
        population.push(b.sum().to_string());
        b.update();
    }

    population
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("out/sim_results.dat")?;
    const REPEATS: i32 = 100;
    let fracs = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    for frac in fracs.iter() {
        for _ in 0..REPEATS {
            let results = run_game(*frac, 100);
            wtr.write_record(results)?;
        }
    }
    wtr.flush()?;
    Ok(())
}
