//! Target module collect the target functions
//! Once there is disput on any header, the relayer should submit the next ethere block target as
//! calculated.
use crate::error::Error;

pub mod half;

pub trait Equation {
    fn calculate(&self, relayed_header: usize, submit_header: usize) -> usize;
}

pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
}
