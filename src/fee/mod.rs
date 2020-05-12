//! Fee module collect the fee functions
//! The fee function will increase the fee to improve speed of the finality.
//!
//! The `Equation` and `ConfigValidate` trait help you to customized your own fee equations.
use crate::error::Error;

pub mod linear;

/// This trait help the main function calculate the fee from the equation
pub trait Equation {
    fn calculate(&self, submit_times: usize) -> f64;
}

/// This trait help the main function validating the parameters when loading yaml
pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
}

impl Equation for f64 {
    fn calculate(&self, _submit_times: usize) -> f64 {
        *self
    }
}
