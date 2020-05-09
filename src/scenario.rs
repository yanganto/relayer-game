use std::str::FromStr;

use serde_derive::Deserialize;
use toml;

use crate::error::Error;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct ScenarioConfig {
    title: Option<String>,
    Dd: usize,
    De: usize,
    Wd: f64,
    We: f64,
    C: usize,
    Md: usize,
    Me: usize,
    relayers: Vec<RelayerConfig>,
}

#[derive(Debug, Deserialize)]
struct RelayerConfig {
    name: Option<String>,
    choice: String,
}

impl FromStr for ScenarioConfig {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c: ScenarioConfig = toml::from_str(s)?;
        if c.Wd < 0.0 {
            return Err(Error::ParameterError("Wd should not be negative"));
        }
        if c.We < 0.0 {
            return Err(Error::ParameterError("We should not be negative"));
        }
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
			Dd = 100
			De = 1000
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
			Dd = 100
			De = 1000
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
			Dd = 100
			De = 1000
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
