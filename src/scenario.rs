//! Scenario Config
//!
//! This config describes the relayer game scenario
//!
//! Some examples are listed in `/scenario `
//!
//! surfix d: block difference between last block number relayed on Darwinia,
//! surfix e: block difference between last related block number of Ethereum
use std::iter::IntoIterator;
use std::str::FromStr;

use serde_derive::Deserialize;
use toml;

use crate::error::Error;
use crate::target::{half::HalfConfig, Equation as TargetEq};
use crate::wait::{linear::LinearConfig, ConfigValidate, Equation as WaitEq};

/// # Scenario Config
/// In this config, the `wait_function`, the initial status, and the `relayers` are defined.
/// The initaial status contains the block difference in the target chain and Darwinia chain.
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ScenarioConfig {
    pub title: Option<String>,
    /// Dd: (optional) the initial block difference between last block number relayed on Darwinia, default 0
    pub Dd: Option<usize>,
    /// De: the initial block difference between last related block number of target chain (for example Ethereum), default 100
    pub De: Option<usize>,

    /// F: The block producing factor for darwinia / ethereum
    /// For example, 2.0 means that darwinia produce 2 blocks and ethereum produce 1 block.
    pub F: Option<f32>,

    /// Once a relayer submit a header and wait the time in blocks after the calculated value from wait
    /// function, Darwinia network will deem this header is valided and become a last relayed header.
    pub wait_function: String,

    /// Once there is disput on any header, the relayer should submit the next ethere block target as
    /// calculated.
    pub target_function: String,

    /// parameters in linear wating
    pub wait_linear: Option<LinearConfig>,

    /// The relayers participate in these game
    /// We suppose that there is always a honest relayer provied by Darwinia,
    /// so after the config correctly imported, the Darwinia relayer will add into.
    pub relayers: Vec<RelayerConfig>,
}

pub struct ScenarioConfigIntoIterator {
    relayers: Vec<RelayerConfig>,
    submit_round: usize,
}

impl IntoIterator for ScenarioConfig {
    type Item = Vec<(String, bool)>;
    type IntoIter = ScenarioConfigIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        ScenarioConfigIntoIterator {
            relayers: self.relayers,
            submit_round: 0,
        }
    }
}

impl ScenarioConfig {
    /// a method to get iterator with less clone, and save memory
    pub fn get_iter(&self) -> ScenarioConfigIntoIterator {
        ScenarioConfigIntoIterator {
            relayers: self.relayers.clone(),
            submit_round: 0,
        }
    }
    pub fn get_wait_equation(&self) -> Result<impl WaitEq, Error> {
        match self.wait_function.to_uppercase().as_str() {
            "LINEAR" => {
                if let Some(e) = self.wait_linear {
                    return Ok(e);
                }
            }
            _ => {
                return Err(Error::ParameterError(
                    "lack prameters for specified wait function",
                ))
            }
        }
        return Err(Error::ParameterError("Wait function absent"));
    }
    pub fn get_target_equation(&self) -> Result<impl TargetEq, Error> {
        match self.target_function.as_str() {
            "half" => return Ok(HalfConfig {}),
            _ => {
                return Err(Error::ParameterError("Target function absent"));
            }
        }
    }
}

impl Iterator for ScenarioConfigIntoIterator {
    type Item = Vec<(String, bool)>;
    /// Return the the relayer is liing in each round
    fn next(&mut self) -> Option<Vec<(String, bool)>> {
        if self.submit_round >= self.relayers[0].choice.len() {
            return None;
        }
        let current_index = self.submit_round;
        self.submit_round += 1;
        Some(
            self.relayers
                .iter()
                .map(|r| {
                    if current_index >= r.choice.len() {
                        Err("this replay has no response")
                    } else {
                        Ok((
                            r.name.clone().unwrap(),
                            r.choice.chars().nth(current_index).unwrap() == 'L',
                        ))
                    }
                })
                .filter_map(Result::ok)
                .collect(),
        )
    }
}

/// RelayerConfig
/// Set up a `name` and the `choice` about the relayer
#[derive(Debug, Deserialize, Clone)]
pub struct RelayerConfig {
    /// Optional field help you to know the relayer in
    pub name: Option<String>,
    /// The client can choice to be Honest(H), Lie(L), No response(N), if the choice is not lone as
    /// other replayer, it will be automaticaly no response
    pub choice: String,
}

impl FromStr for ScenarioConfig {
    type Err = Error;
    /// Pase from the scenario toml file as listed in `/scenario `
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c: ScenarioConfig = toml::from_str(s)?;
        c.wait_function.make_ascii_uppercase();
        match c.wait_function.as_str() {
            "LINEAR" => {
                if let Some(l) = &c.wait_linear {
                    l.validate()?
                }
            }
            _ => {
                return Err(Error::ParameterError("waiting function not support"));
            }
        };

        let mut max_chose = 0;
        for (i, r) in c.relayers.iter_mut().enumerate() {
            if let Some(n) = &r.name {
                if n.to_uppercase() == "DARWINIA".to_string() {
                    return Err(Error::ParameterError(
                        "Darwinia relayer alway honest, you do not need modify it",
                    ));
                }
            } else {
                r.name = Some(format!(" {}", i));
            };
            // TODO: check name should not use number
            r.choice.make_ascii_uppercase();
            max_chose = std::cmp::max(max_chose, r.choice.len());
            for c in r.choice.chars() {
                if c != 'H' && c != 'L' && c != 'N' {
                    return Err(Error::ParameterError("relayer chose must be 'H', 'L', 'N'"));
                }
            }
        }
        let mut relayers = vec![RelayerConfig {
            name: Some("Darwinia".to_string()),
            choice: "H".repeat(max_chose + 1),
        }];
        relayers.append(&mut c.relayers);
        c.relayers = relayers;
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TOML_CONFIG: &'static str = r#"
			wait_function = "linear"
			target_function = "half"

			Dd = 100
			De = 1000

			[wait_linear]
			Wd = 0.0
			We = 0.0
			C  = 1
			Md = 100
			Me = 100
			B = 1
			T = 10

			[[relayers]]
			name = "Evil"
			choice = "LL"

			[[relayers]]
			name = "Honest"
			choice = "H"
		"#;
    #[test]
    fn test_parse_toml() {
        let c: ScenarioConfig = toml::from_str(TOML_CONFIG).unwrap();
        assert_eq!(c.relayers.len(), 2);
        assert_eq!(c.relayers[1].choice, "H");
    }
    #[test]
    fn test_from_toml_str() {
        let c = <ScenarioConfig>::from_str(TOML_CONFIG);
        assert!(c.is_ok());
        let c = c.unwrap();
        assert_eq!(c.relayers.len(), 3);

        // Darwinia relayer should be added automatically
        assert_eq!(c.relayers[0].name, Some("Darwinia".to_string()));
        assert_eq!(c.relayers[0].choice, "HHH");
        assert_eq!(c.relayers[1].choice, "LL");
        assert_eq!(c.relayers[2].choice, "H");
    }
    #[test]
    fn test_iterate_from_scenario() {
        let c = <ScenarioConfig>::from_str(TOML_CONFIG).unwrap();
        let mut i = c.into_iter();
        assert_eq!(
            i.next(),
            Some(vec![
                ("Darwinia".to_string(), false),
                ("Evil".to_string(), true),
                ("Honest".to_string(), false)
            ])
        );
        assert_eq!(
            i.next(),
            Some(vec![
                ("Darwinia".to_string(), false),
                ("Evil".to_string(), true),
            ])
        );
        assert_eq!(i.next(), Some(vec![("Darwinia".to_string(), false),]));
        assert_eq!(i.next(), None);
    }
    #[test]
    fn test_from_error_toml_str() {
        let c = <ScenarioConfig>::from_str(
            r#"
			wait_function = "linear"
			target_function = "half"

			[wait_linear]
			Wd = -10
			We = 0.0
			C  = 1
			Md = 100
			Me = 100
			B = 1
			T = 10

			[[relayers]]
			name = "Honest"
			choice = "HHHHHHH"
		"#,
        );
        assert!(format!("{:?}", c).contains("Wd should not be negative"));
    }
    #[test]
    fn test_auto_upper_case() {
        let c = <ScenarioConfig>::from_str(
            r#"
			wait_function = "linear"
			target_function = "half"

			[wait_linear]
			Wd = 0.0
			We = 0.0
			C  = 1
			Md = 100
			Me = 100
			B = 1
			T = 10

			[[relayers]]
			name = "Honest"
			choice = "hhhhhhh"
			"#,
        )
        .unwrap();
        assert_eq!(c.relayers[1].choice, "HHHHHHH");
    }
}
