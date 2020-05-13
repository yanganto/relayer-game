//! Linear Equation for the fee function
use crate::error::Error;
use crate::fee::{ConfigValidate, Equation};
use serde_derive::Deserialize;

/// # Linear fee function
/// Here is the linear equation  
/// fee of submit = min(W * B, M)) + C
#[allow(non_snake_case)]
#[derive(Default, Debug, Deserialize, Copy, Clone)]
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
    fn apply_patch(&mut self, k: &str, v: &str) -> Result<(), Error> {
        match k {
            "C" => self.C = v.parse::<f64>()?,
            "W" => self.W = v.parse::<f64>()?,
            "M" => self.M = v.parse::<f64>()?,
            _ => {
                return Err(Error::PatchParameterError(
                    "parameter not correct".to_string(),
                ))
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_apply_patch() {
        let mut c = LinearConfig::default();
        c.apply_patch("C", "9.9").unwrap();
        c.apply_patch("W", "1.234").unwrap();
        c.apply_patch("M", "8.8").unwrap();
        assert_eq!(c.C, 9.9);
        assert_eq!(c.W, 1.234);
        assert_eq!(c.M, 8.8);
    }
}
