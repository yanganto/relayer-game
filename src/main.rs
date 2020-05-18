//! Relayer Game Simulation Tool
//!
//! This tool load the different `scenario` with different `challenge_equation`, listed in the
//! `challenge` module, with different `bond_equation`, listed in the `bond` module,
//! and simulate the result, let people know more about the time delay in blocks and
//! the reward distribution.
//!

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use clap::{App, Arg};
use colored::Colorize;

use crate::target::Equation as TargetEq;

mod bond;
mod chain;
mod challenge;
mod error;
#[cfg(feature = "plot")]
mod plot;
mod reward;
mod scenario;
mod target;

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
    let challenge_eq = config.get_challenge_equation()?;
    let target_eq = config.get_target_equation()?;
    let reward_eq = config.get_reward_equation()?;
    let bond_eq = config.get_bond_equation()?;
    let mut chains_status: chain::ChainsStatus = config.into();
    let darwinia_start_block = chains_status.darwinia_block_hight;

    // record variable pre submition
    let mut challenge_times = Vec::<f64>::new();
    let mut bonds = Vec::<f64>::new();

    let mut reward_actions = Vec::<chain::Reward>::new();
    let mut reward_from_previous_round = 0f64;
    let mut rp = scenario::RelayPositions::default();
    rp.relay_blocks
        .push(chains_status.submit_target_ethereum_block);

    while let Some(relayer_subitions) = iterator.next() {
        let bond = bond_eq.calculate(iterator.submit_round);
        bonds.push(bond);
        let submition_times = chains_status.submitions.len();
        let last_relayed_block = if submition_times > 0 {
            chains_status.submitions[submition_times - 1]
        } else {
            (0, 0)
        };
        let challenge_time = if last_relayed_block.1 > chains_status.submit_target_ethereum_block {
            challenge_eq.calculate(
                chains_status.darwinia_block_hight - last_relayed_block.0,
                last_relayed_block.1 - chains_status.submit_target_ethereum_block,
            )
        } else {
            challenge_eq.calculate(
                chains_status.darwinia_block_hight - last_relayed_block.0,
                chains_status.submit_target_ethereum_block - last_relayed_block.1,
            )
        };
        challenge_times.push(challenge_time as f64);

        let total_lie_relayer = relayer_subitions.iter().filter(|r| r.1).count();
        let mut r = reward_eq.calculate(
            reward_from_previous_round,
            total_lie_relayer as f64 * bond,
            bond,
            relayer_subitions
                .iter()
                .filter(|r| !r.1)
                .map(|r| r.0.clone())
                .collect(),
        );
        reward_from_previous_round = r.0;
        reward_actions.append(&mut r.1);

        if debug {
            print!("{}", format!("{}", chains_status.fmt_status()).cyan());
            println!("\tSubmission Plot: {}", rp.plot());
            print!("\tSubmission(Bond: {}): ", bond);
            for (r, lie) in relayer_subitions.iter() {
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

        chains_status.submit(
            relayer_subitions,
            bond,
            challenge_time,
            target_eq.calculate(0, chains_status.submit_target_ethereum_block),
        );
        rp.relay_blocks
            .push(chains_status.submit_target_ethereum_block);

        // TODO: make this as an option
        chains_status.should_balance();

        if debug {
            println!(
                "\tNext Etherem Target Block: {}",
                chains_status.submit_target_ethereum_block
            );
            println!("\tChallenge Time: {} blocks", challenge_time);
            println!("\tRelayer Status: {}", chains_status.fmt_relayers_status());
            println!(
                "\tSubmit Bond Pool Status: {}",
                chains_status.submit_bond_pool
            );
        }

        if 0 == total_lie_relayer {
            break;
        }
    }
    let max_bond_value = chains_status.submit_bond_pool;
    chains_status.reward(reward_actions);

    #[cfg(feature = "plot")]
    plot::draw("Challenge Times", iterator.submit_round, challenge_times)
        .map_err(|e| error::Error::PlotError(format!("{:?}", e)))?;

    #[cfg(feature = "plot")]
    plot::draw("Bonds", iterator.submit_round, bonds)
        .map_err(|e| error::Error::PlotError(format!("{:?}", e)))?;

    println!("Final {}", chains_status);
    println!(
        "Duration: {} blocks,  Max Bond Value: {}",
        chains_status.darwinia_block_hight - darwinia_start_block,
        max_bond_value
    );

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
        matches.values_of("patch").unwrap_or_default().collect(),
        matches.is_present("verbose"),
    ) {
        Err(e) => println!("{}", e),
        _ => {}
    }
}
