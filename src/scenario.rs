use std::str::FromStr;

use serde_derive::Deserialize;
use toml;

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
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: ScenarioConfig = toml::from_str(s)?;
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
        assert_eq!(c.relayers.len(), 2);
        assert_eq!(c.relayers[1].choice, "TTTTTTT");
    }
}
