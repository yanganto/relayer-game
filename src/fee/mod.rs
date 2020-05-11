use crate::error::Error;

pub trait Equation {
    fn calculate(&self, darwinia_distance: usize, ethereum_distance: usize) -> f64;
}

pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
}

impl Equation for f64 {
    fn calculate(&self, _darwinia_distance: usize, _ethereum_distance: usize) -> f64 {
        *self
    }
}
