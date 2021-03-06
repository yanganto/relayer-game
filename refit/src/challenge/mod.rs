//! Challenge module collect the challenge time equations
//! Once a relayer submit a header and wait a challenge time in blocks after the calculated value from challenge
//! function, Darwinia network will deem this header is valided and become a best header.
//! There is only linear module at first.
//!
//! The `Equation` and `ConfigValidate` trait help you to customized your own challenge equations.
use crate::error::Error;

pub mod linear;

/// This trait help the main function calculate the bond from the equation
pub trait Equation {
    fn calculate(&self, darwinia_distance: usize, ethereum_distance: usize) -> usize;
}

/// This trait help the main function
/// - validating the parameters when loading yaml
/// - apply patch when user pass it as option `p` from command line
pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
    fn apply_patch(&mut self, k: &str, v: &str) -> Result<(), Error>;
}

impl Equation for usize {
    fn calculate(&self, _darwinia_distance: usize, _ethereum_distance: usize) -> usize {
        *self
    }
}
