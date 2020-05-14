//! Bond module collect the bond equations
//! The bond function will increase the bond to improve speed of the finality.
//!
//! The `Equation` and `ConfigValidate` trait help you to customized your own bond equations.
use crate::error::Error;

pub mod linear;

/// This trait help the main function calculate the bond values for each round from the equation
pub trait Equation {
    fn calculate(&self, submit_times: usize) -> f64;
}

/// This trait help the main function
/// - validating the parameters when loading yaml
/// - apply patch when user pass it as option `p` from command line
pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
    fn apply_patch(&mut self, k: &str, v: &str) -> Result<(), Error>;
}

impl Equation for f64 {
    fn calculate(&self, _submit_times: usize) -> f64 {
        *self
    }
}
