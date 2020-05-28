//! Sample module collect the sampling functions
//! Once there is disput on any header, the relayer should submit the next sampling ethere block as
//! calculated. Namely, the sampling function will point out the next sampling block
//!
//! The `Equation` and `ConfigValidate` trait help you to customized your own sampling equations.
use crate::error::Error;

pub mod half;

/// This trait help the main function calculate the next sampling block from the equation
pub trait Equation {
    fn calculate(&self, relayed_header: usize, submit_header: usize) -> usize;
}

/// This trait help the main function validating the parameters when loading yaml
pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
}
