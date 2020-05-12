//! Fee module collect the fee functions
//! The fee function will increase the fee to improve speed of the finality.
//!
use crate::error::Error;

pub mod linear;

pub trait Equation {
    fn calculate(&self, submit_times: usize) -> f64;
}

pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
}

impl Equation for f64 {
    fn calculate(&self, _submit_times: usize) -> f64 {
        *self
    }
}
