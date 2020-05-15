//! Reward module collect the reward equations
//! The bond function will increase the bond to improve speed of the finality.
//!
//! The `Equation` and `ConfigValidate` trait help you to customized your own bond equations.
use crate::chain::Reward;
use crate::error::Error;

pub mod split;
pub mod treasury_last;

/// This trait help the main function calculate the Reward and the reserve slash
pub trait Equation {
    fn calculate(
        &self,
        previous_slash: f64,
        curent_slash: f64,
        curent_bond: f64,
        honest_relayers: Vec<String>,
    ) -> (f64, Vec<Reward>);
}

/// This trait help the main function
/// - validating the parameters when loading yaml
/// - apply patch when user pass it as option `p` from command line
pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
    fn apply_patch(&mut self, k: &str, v: &str) -> Result<(), Error>;
}
