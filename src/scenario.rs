//! Scenario Config
//!
//! This config describes the relayer game scenario
//!
//! Some examples are listed in `/scenario `
//!
use std::str::FromStr;

use serde_derive::Deserialize;
use toml;

use crate::error::Error;
use crate::wait::{linear::LinearConfig, ConfigValidate};

/// # Scenario Config
/// In this config, the `wait_function`, the initial status, and the `relayers` are defined.
/// The initaial status contains the block difference in the target chain and Darwinia chain.
/// Once a relayer submit a header and wait the time in blocks after the calculated value from wait
/// function, Darwinia network will deem this header is valided and become a best header.
/// We suppose that there is always a honest relayer provied by Darwinia,
/// so after the config correctly imported, the Darwinia relayer will add into.
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ScenarioConfig {
    pub title: Option<String>,
    /// Di: the initial block difference between last block number relayed on Darwinia,
    pub Di: usize,
    /// Ei: the initial block difference between last related block number of target chain (for example Ethereum)
    pub Ei: usize,
    pub wait_function: String,
    pub linear: Option<LinearConfig>,
    pub relayers: Vec<RelayerConfig>,
}

/// RelayerConfig
/// Set up a `name` and the `choice` about the relayer
#[derive(Debug, Deserialize)]
pub struct RelayerConfig {
    /// Optional field help you to know the relayer in
    pub name: Option<String>,
    /// The client can choice to be honest(T), lie(F), No response(N), if the choice is not lone as
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
                if let Some(l) = &c.linear {
                    l.validate()?
                }
            }
            _ => {
                return Err(Error::ParameterError("waiting function not support"));
            }
        };

        let mut max_chose = 0;
        for r in c.relayers.iter_mut() {
            r.choice.make_ascii_uppercase();
            max_chose = std::cmp::max(max_chose, r.choice.len());
            for c in r.choice.chars() {
                if c != 'T' && c != 'F' && c != 'N' {
                    return Err(Error::ParameterError("relayer chose must be 'T', 'F', 'N'"));
                }
            }
        }
        c.relayers.push(RelayerConfig {
            name: Some("Darwinia".to_string()),
            choice: "T".repeat(max_chose),
        });
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TOML_CONFIG: &'static str = r#"
			title = "title"
			wait_function = "linear"
			Di = 100
			Ei = 1000

			[linear]
			Wd = 0.0
			We = 0.0
			C  = 1
			Md = 100
			Me = 100
			B = 1
			T = 10

			[[relayers]]
			name = "evil"
			choice = "FFFFFF" 

			[[relayers]]
			name = "Honest"
			choice = "TTTTTTT" 
		"#;
    #[test]
    fn test_parse_toml() {
        let c: ScenarioConfig = toml::from_str(TOML_CONFIG).unwrap();
        assert_eq!(c.relayers.len(), 2);
        assert_eq!(c.relayers[1].choice, "TTTTTTT");
    }
    #[test]
    fn test_from_toml_str() {
        let c = <ScenarioConfig>::from_str(TOML_CONFIG);
        assert!(c.is_ok());
        let c = c.unwrap();
        assert_eq!(c.relayers.len(), 3);
        assert_eq!(c.relayers[1].choice, "TTTTTTT");
        assert_eq!(c.relayers[2].name, Some("Darwinia".to_string()));
        assert_eq!(c.relayers[2].choice, "TTTTTTT");
    }
    #[test]
    fn test_from_error_toml_str() {
        let c = <ScenarioConfig>::from_str(
            r#"
			title = "title"
			wait_function = "linear"
			Di = 100
			Ei = 1000

			[linear]
			Wd = -10
			We = 0.0
			C  = 1
			Md = 100
			Me = 100
			B = 1
			T = 10

			[[relayers]]
			name = "Honest"
			choice = "TTTTTTT" 
		"#,
        );
        assert!(format!("{:?}", c).contains("Wd should not be negative"));
    }
    #[test]
    fn test_auto_upper_case() {
        let c = <ScenarioConfig>::from_str(
            r#"
			title = "title"
			wait_function = "linear"
			Di = 100
			Ei = 1000

			[linear]
			Wd = 0.0
			We = 0.0
			C  = 1
			Md = 100
			Me = 100
			B = 1
			T = 10

			[[relayers]]
			name = "Honest"
			choice = "ttttttt" 
		"#,
        )
        .unwrap();
        assert_eq!(c.relayers[0].choice, "TTTTTTT");
    }
}
