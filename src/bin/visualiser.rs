use cgol::Board;
use clap::App;
use std::{thread, time::Duration};

fn main() {

    let matches = App::new("Conways Game of Life: Visualiser")
        .version("0.1.0")
        .author("Joshua Read")
        .about("Visualiser for Conways Game of Life.")
        .args_from_usage(
            "-f --fraction=f64   'Set the fraction of living cells'
            -m, --width=u32      'Sets width of board (default=50)'
            -n, --height=u32     'Sets height of board (default=20)'")
        .get_matches();

    let fraction: f64 = matches.value_of("fraction").unwrap_or("0.4")
        .parse().unwrap();
    let width: i32 = matches.value_of("width").unwrap_or("50")
        .parse().unwrap();
    let height: i32 = matches .value_of("height").unwrap_or("20")
        .parse().unwrap();

    let mut b = Board::from_probability(width, height, fraction);

    loop {
        print!("\x1B[2J"); // clears terminal
        println!("{}", b);
        println!("{} cells alive.", b.sum());
        b.update();
        thread::sleep(Duration::from_secs(1));
    }
}
