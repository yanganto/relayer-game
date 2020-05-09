//! Relayer Game Simulation Tool
//!
//! This tool load the different `scenario` with different `wait_fucntion`, listed in the `wait`
//! module, and simulate the result, let people know more about the time delay in blocks and
//! the reward distribution.
//!
use clap::App;

mod error;
mod scenario;
mod wait;

fn main() {
    let _matches = App::new("Relayer Game")
        .about("Relayer Gaming Simulation Tool")
        .arg("<input> 'scenario yaml file'")
        .arg("-v, --verbose 'verbose output'")
        .get_matches();
}
