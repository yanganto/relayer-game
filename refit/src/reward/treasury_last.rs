use crate::chain::{Reward, RewardFrom};
use crate::error::Error;
use crate::reward::{ConfigValidate, Equation};
use serde_derive::Deserialize;

/// # Treasure Last reward equation
/// slash value of each submit round will pay for the honest relayer in the same round
/// and the honest relayers in the last round will be payed from treasury
#[allow(non_snake_case)]
#[derive(Default, Debug, Deserialize, Copy, Clone)]
pub struct TreasureLastConfig {
    /// C: the value will use to pay the honest relayers in the last round
    C: f64,
}

impl ConfigValidate for TreasureLastConfig {
    fn validate(&self) -> Result<(), Error> {
        if self.C < 0.0 {
            return Err(Error::ParameterError("C should be greater than 0"));
        }
        Ok(())
    }
    fn apply_patch(&mut self, k: &str, v: &str) -> Result<(), Error> {
        if k == "C" {
            self.C = v.parse::<f64>()?;
            Ok(())
        } else {
            Err(Error::PatchParameterError(
                "parameter not correct".to_string(),
            ))
        }
    }
}

impl Equation for TreasureLastConfig {
    fn calculate(
        &self,
        _previous_slash: f64,
        current_slash: f64,
        current_bond: f64,
        honest_relayers: Vec<String>,
    ) -> (f64, Vec<Reward>) {
        let rewards = if current_slash == 0f64 {
            let share_for_honest_relayer = self.C / honest_relayers.len() as f64;
            honest_relayers
                .into_iter()
                .map(|r| Reward {
                    from: RewardFrom::Treasure,
                    to: r,
                    value: share_for_honest_relayer + current_bond,
                })
                .collect()
        } else {
            let share_for_honest_relayer = current_slash / honest_relayers.len() as f64;
            honest_relayers
                .into_iter()
                .map(|r| Reward {
                    from: RewardFrom::Slash,
                    to: r,
                    value: share_for_honest_relayer + current_bond,
                })
                .collect()
        };
        (0f64, rewards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_apply_patch() {
        let mut c = TreasureLastConfig::default();
        c.apply_patch("C", "10.0").unwrap();
        assert_eq!(c.C, 10.0);
    }
}
