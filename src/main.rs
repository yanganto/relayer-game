//! Relayer Game Simulation Tool
//!
//! This tool load the different `scenario` with different `wait_fucntion`, listed in the `wait`
//! module, and simulate the result, let people know more about the time delay in blocks and
//! the reward distribution.
//!
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use clap::App;

mod chain;
mod error;
mod scenario;
mod wait;

fn simulate_from_scenario(file_name: &str, debug: bool) -> Result<(), error::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = <scenario::ScenarioConfig>::from_str(&contents)?;
    let mut iterator = config.get_iter();
    let mut chains_status: chain::ChainsStatus = config.into();
    while let Some(relayer_sumbitions) = iterator.next() {
        if debug {
            print!("Sumitions {}", chains_status.fmt_status());
            print!(" Submitions ");
            for (r, lie) in relayer_sumbitions.iter() {
                print!("{}", r);
                if *lie {
                    print!("(lie)");
                }
                print!(" ");
            }
            print!("\n");
        }
        chains_status.submit(relayer_sumbitions, 10.0);
        if debug {
            println!("{}", chains_status.fmt_relayers_status());
        }
    }
    chains_status.reward_honest_relayers();
    println!("Final {}", chains_status);
    Ok(())
}

fn main() {
    let matches = App::new("Relayer Game")
        .about("Relayer Gaming Simulation Tool")
        .arg("<scenario> 'scenario yaml file'")
        .arg("-v, --verbose 'show the detail of each submit'")
        .get_matches();
    match simulate_from_scenario(
        matches.value_of("scenario").unwrap(),
        matches.is_present("verbose"),
    ) {
        Err(e) => println!("{}", e),
        _ => {}
    }
}
