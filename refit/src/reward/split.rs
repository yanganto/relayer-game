use crate::chain::{Reward, RewardFrom};
use crate::error::Error;
use crate::reward::{ConfigValidate, Equation};
use serde_derive::Deserialize;

/// # Split reward equation
/// slash value of submit round will take P as reward in current round, and leave (1-P) for the next
/// round
#[allow(non_snake_case)]
#[derive(Default, Debug, Deserialize, Copy, Clone)]
pub struct SplitConfig {
    /// P: the portion use in the current round, else will leave to next round
    P: f64,
}

impl ConfigValidate for SplitConfig {
    fn validate(&self) -> Result<(), Error> {
        if self.P > 1.0 {
            return Err(Error::ParameterError("P should not be greater than 1"));
        }
        Ok(())
    }
    fn apply_patch(&mut self, k: &str, v: &str) -> Result<(), Error> {
        if k == "P" {
            self.P = v.parse::<f64>()?;
            Ok(())
        } else {
            Err(Error::PatchParameterError(
                "parameter not correct".to_string(),
            ))
        }
    }
}

impl Equation for SplitConfig {
    fn calculate(
        &self,
        previous_slash: f64,
        current_slash: f64,
        current_bond: f64,
        honest_relayers: Vec<String>,
    ) -> (f64, Vec<Reward>) {
        let remind = (1.0 - self.P) * current_slash;
        let slash = previous_slash + current_slash - remind;
        let share_for_honest_relayer = slash / honest_relayers.len() as f64;
        (
            remind,
            honest_relayers
                .into_iter()
                .map(|r| Reward {
                    from: RewardFrom::Slash,
                    to: r,
                    value: share_for_honest_relayer + current_bond,
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_apply_patch() {
        let mut c = SplitConfig::default();
        c.apply_patch("P", "0.9").unwrap();
        assert_eq!(c.P, 0.9);
    }
}
