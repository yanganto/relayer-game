use crate::error::Error;

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
