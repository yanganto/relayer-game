//! Target module collect the target functions
//! Once there is disput on any header, the relayer should submit the next ethere block target as
//! calculated.
//!
//! The `Equation` and `ConfigValidate` trait help you to customized your own target equations.
use crate::error::Error;

pub mod half;

/// This trait help the main function calculate the target from the equation
pub trait Equation {
    fn calculate(&self, relayed_header: usize, submit_header: usize) -> usize;
}

/// This trait help the main function validating the parameters when loading yaml
pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
}
