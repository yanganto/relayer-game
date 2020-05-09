//! Linear Equation is the simplest wait function
use crate::error::Error;
use crate::wait::ConfigValidate;
use serde_derive::Deserialize;

/// # Linear waiting function
/// Here is the linear equation  
/// waiting blocks = min(Wd * D, Md) + min(We * E, Me)) + C
///
/// d: the parameters affect by the parameters on Darwinia network
/// e: the parameters affect by the parameters on target network(for example Ethereum)
///
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
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
}
