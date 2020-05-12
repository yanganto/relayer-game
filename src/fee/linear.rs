//! Linear Equation for the fee function
use crate::error::Error;
use crate::fee::{ConfigValidate, Equation};
use serde_derive::Deserialize;

/// # Linear fee function
/// Here is the linear equation  
/// fee of submit = min(W * B, M)) + C
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct LinearConfig {
    /// W: the weights of submit times
    W: f64,
    /// The contance
    C: f64,
    /// The upper limitation for submit fee
    M: f64,
}

impl ConfigValidate for LinearConfig {
    fn validate(&self) -> Result<(), Error> {
        if self.W < 0.0 {
            return Err(Error::ParameterError("W should not be negative"));
        }
        Ok(())
    }
}

impl Equation for LinearConfig {
    /// fee = min(W * E, M) + C
    fn calculate(&self, submit_round: usize) -> f64 {
        let weight_part = self.W * submit_round as f64;
        if weight_part > self.M {
            return self.M + self.C;
        } else {
            return weight_part + self.C;
        }
    }
}
