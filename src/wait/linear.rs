//! Linear Equation is the simplest wait function
use std::cmp::min;

use crate::error::Error;
use crate::wait::{ConfigValidate, Equation};
use serde_derive::Deserialize;

/// # Linear waiting function
/// Here is the linear equation  
/// waiting blocks = min(Wd * D, Md) + min(We * E, Me)) + C
///
/// d: the parameters affect by the parameters on Darwinia network
/// e: the parameters affect by the parameters on target network(for example Ethereum)
///
#[allow(non_snake_case)]
#[derive(Default, Debug, Deserialize, Copy, Clone)]
pub struct LinearConfig {
    /// Md: the max value about D portion
    Wd: f64,
    /// Me: the max value about E portion
    We: f64,
    /// The contance
    C: usize,
    /// The upper limitation for Darwinia part
    Md: usize,
    /// The upper limitation for target chain part
    Me: usize,
}

impl ConfigValidate for LinearConfig {
    fn validate(&self) -> Result<(), Error> {
        if self.Wd < 0.0 {
            return Err(Error::ParameterError("Wd should not be negative"));
        }
        if self.We < 0.0 {
            return Err(Error::ParameterError("We should not be negative"));
        }
        Ok(())
    }
    fn apply_patch(&mut self, k: &str, v: &str) -> Result<(), Error> {
        match k {
            "Wd" => self.Wd = v.parse::<f64>()?,
            "We" => self.We = v.parse::<f64>()?,
            "Md" => self.Md = v.parse::<usize>()?,
            "Me" => self.Me = v.parse::<usize>()?,
            "C" => self.C = v.parse::<usize>()?,
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
    /// waiting block = int(min(Wd * D, Md) + min(We * E, Me)) + C
    fn calculate(&self, darwinia_distance: usize, ethereum_distance: usize) -> usize {
        min((self.Wd * darwinia_distance as f64) as usize, self.Md)
            + min((self.Wd * ethereum_distance as f64) as usize, self.Md)
            + self.C
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_apply_patch() {
        let mut c = LinearConfig::default();
        c.apply_patch("C", "9").unwrap();
        c.apply_patch("Wd", "1.234").unwrap();
        c.apply_patch("We", "4.56").unwrap();
        c.apply_patch("Md", "8").unwrap();
        c.apply_patch("Me", "7").unwrap();
        assert_eq!(c.C, 9);
        assert_eq!(c.Wd, 1.234);
        assert_eq!(c.We, 4.56);
        assert_eq!(c.Md, 8);
        assert_eq!(c.Me, 7);
    }
}
