//! Wait module collect the wait functions
//! Once a relayer submit a header and wait the time in blocks after the calculated value from wait
//! function, Darwinia network will deem this header is valided and become a best header.
//! There is only linear module at first.
use crate::error::Error;

pub mod linear;

pub trait Equation {
    fn calculate(&self, darwinia_distance: usize, ethereum_distance: usize) -> usize;
}

pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
}

impl Equation for usize {
    fn calculate(&self, _darwinia_distance: usize, _ethereum_distance: usize) -> usize {
        *self
    }
}
