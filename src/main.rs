//! Relayer Game Simulation Tool
//!
//! This tool load the different `scenario` with different `wait_fucntion`, listed in the `wait`
//! module, and simulate the result, let people know more about the time delay in blocks and
//! the reward distribution.
//!

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use clap::{App, Arg};
use colored::Colorize;

use crate::target::Equation as TargetEq;

mod chain;
mod error;
mod fee;
mod scenario;
mod target;
mod wait;

fn simulate_from_scenario(
    file_name: &str,
    patches: Vec<&str>,
    debug: bool,
) -> Result<(), error::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut config = <scenario::ScenarioConfig>::from_str(&contents)?;
    config.apply_patch(patches)?;

    if let Some(t) = &config.title {
        println!("{}", t.white());
    }

    let mut iterator = config.get_iter();
    let wait_eq = config.get_wait_equation()?;
    let target_eq = config.get_target_equation()?;
    let fee_eq = config.get_fee_equation()?;
    let mut chains_status: chain::ChainsStatus = config.into();
    while let Some(relayer_sumbitions) = iterator.next() {
        if debug {
            print!("{}", format!("{}", chains_status.fmt_status()).cyan());
            print!(
                "\tSubmitions(Fee: {}): ",
                fee_eq.calculate(iterator.submit_round)
            );
            for (r, lie) in relayer_sumbitions.iter() {
                print!("{}", r);
                if *lie {
                    print!("(lie)");
                } else {
                    print!("(honest)");
                }
                print!(" ");
            }
            print!("\n");
        }
        let ethereum_distance =
            if chains_status.last_relayed_block.1 > chains_status.submit_target_ethereum_block {
                chains_status.last_relayed_block.1 - chains_status.submit_target_ethereum_block
            } else {
                chains_status.submit_target_ethereum_block - chains_status.last_relayed_block.1
            };
        chains_status.submit(
            relayer_sumbitions,
            fee_eq.calculate(iterator.submit_round),
            wait_eq.calculate(
                chains_status.darwinia_block_hight - chains_status.last_relayed_block.0,
                ethereum_distance,
            ),
            target_eq.calculate(
                chains_status.submit_target_ethereum_block,
                chains_status.last_relayed_block.1,
            ),
        );

        // TODO: make this as an option
        chains_status.should_balance();

        if debug {
            println!(
                "\tNext Etherem Target Block: {}",
                chains_status.submit_target_ethereum_block
            );
            print!("\tRelayer Status: ");
            println!("{}", chains_status.fmt_relayers_status());
            println!(
                "\tSubmit Fee Pool Status: {}",
                chains_status.submit_fee_pool
            );
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
        .arg(
            Arg::with_name("patch")
                .multiple(true)
                .short('p')
                .takes_value(true),
        )
        .get_matches();
    match simulate_from_scenario(
        matches.value_of("scenario").unwrap(),
        matches.values_of("patch").unwrap().collect(),
        matches.is_present("verbose"),
    ) {
        Err(e) => println!("{}", e),
        _ => {}
    }
}
