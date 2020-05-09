use clap::App;

mod scenario;

fn main() {
    let _matches = App::new("Relayer Game")
        .about("Relayer Gaming Simulation Tool")
        .arg("<input> 'scenario yaml file'")
        .arg("-v, --verbose 'verbose output'")
        .get_matches();
}
