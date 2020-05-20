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

use crate::bond::{
    linear::LinearConfig as BondLinear, ConfigValidate as BondVali, Equation as BondEq,
};
use crate::challenge::{
    linear::LinearConfig as ChallengeLinear, ConfigValidate as ChallengeVali,
    Equation as ChallengeEq,
};
use crate::error::Error;
use crate::reward::{
    split::SplitConfig, treasury_last::TreasureLastConfig, ConfigValidate as RewardVali,
    Equation as RewardEq,
};
use crate::target::{half::HalfConfig, Equation as TargetEq};

/// # Scenario Config
/// In this config, the `challenge_function`, the initial status, and the `relayers` are defined.
/// The initial status contains the block difference in the target chain and Darwinia chain.
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
    pub F: Option<f64>,

    /// Once a relayer submit a header and wait the challenge time in blocks after the calculated value equation from challenge
    /// function, Darwinia network will deem this header is validated and become a last relayed header.
    pub challenge_function: String,

    /// Once there is dispute on any header, the relayer should submit the next ethereum block target as
    /// calculated.
    pub target_function: String,

    /// The submit bond of function
    pub bond_function: String,

    /// The reward of function
    pub reward_function: String,

    /// parameters in linear waiting
    pub challenge_linear: Option<ChallengeLinear>,

    /// parameters in linear waiting
    pub bond_linear: Option<BondLinear>,

    /// parameters in split reward
    pub reward_split: Option<SplitConfig>,

    /// parameters in treasury reward the last submit round
    pub reward_treasury_last: Option<TreasureLastConfig>,

    /// The relayers participate in these game
    /// We suppose that there is always a honest relayer provided by Darwinia,
    /// so after the config correctly imported, the Darwinia relayer will add into.
    pub relayers: Vec<RelayerConfig>,

    /// current challenge fee is the same with bond function
    /// challenger list (current implementation allow only one challenger)
    pub challengers: Option<Vec<RelayerConfig>>,
}

#[derive(Default)]
pub struct RelayPositions {
    pub geneisis: usize,
    pub relay_blocks: Vec<usize>,
}

impl RelayPositions {
    pub fn plot(&self) -> String {
        let mut output = "G".to_string();
        let max_relay_block = if self.relay_blocks.len() > 0 {
            self.relay_blocks[0]
        } else {
            1
        };
        let block_indece: Vec<usize> = self
            .relay_blocks
            .clone()
            .into_iter()
            .map(|v| ((v as f64 / max_relay_block as f64) * 64.0) as usize)
            .collect();
        for i in 1..65 {
            if let Some(idx) = block_indece.iter().position(|&x| x == i) {
                output.push_str(&format!("{}", idx + 1));
            } else {
                output.push_str("=");
            }
        }
        output.push_str("==>");
        output
    }
}

pub struct ScenarioConfigIntoIterator {
    relayers: Vec<RelayerConfig>,
    pub submit_round: usize,
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
    pub fn get_challenge_equation(&self) -> Result<Box<dyn ChallengeEq>, Error> {
        if let Ok(i) = self.challenge_function.as_str().parse::<usize>() {
            return Ok(Box::new(i));
        }
        match self.challenge_function.to_uppercase().as_str() {
            "LINEAR" => {
                if let Some(w) = self.challenge_linear {
                    return Ok(Box::new(w));
                }
            }
            _ => {
                return Err(Error::ParameterError("Challenge function not support"));
            }
        }
        return Err(Error::ParameterError(
            "lack prameters for specified challenge function",
        ));
    }
    pub fn get_target_equation(&self) -> Result<impl TargetEq, Error> {
        match self.target_function.to_uppercase().as_str() {
            "HALF" => return Ok(HalfConfig {}),
            _ => {
                return Err(Error::ParameterError("Target function absent"));
            }
        }
    }
    pub fn get_bond_equation(&self) -> Result<Box<dyn BondEq>, Error> {
        if let Ok(i) = self.bond_function.as_str().parse::<f64>() {
            return Ok(Box::new(i));
        }
        match self.bond_function.to_uppercase().as_str() {
            "LINEAR" => {
                if let Some(f) = self.bond_linear {
                    return Ok(Box::new(f));
                }
            }
            _ => {
                return Err(Error::ParameterError("Bond function not support"));
            }
        }
        return Err(Error::ParameterError(
            "lack prameters for specified bond function",
        ));
    }
    pub fn get_reward_equation(&self) -> Result<Box<dyn RewardEq>, Error> {
        match self.reward_function.to_uppercase().as_str() {
            "SPLIT" => {
                if let Some(f) = self.reward_split {
                    return Ok(Box::new(f));
                }
            }
            "TREASURY_LAST" => {
                if let Some(f) = self.reward_treasury_last {
                    return Ok(Box::new(f));
                }
            }
            _ => {
                return Err(Error::ParameterError("Reward function absent"));
            }
        }
        return Err(Error::ParameterError(
            "lack prameters for specified reward function",
        ));
    }
    pub fn apply_patch(&mut self, patches: Vec<&str>) -> Result<(), Error> {
        let split_with_equation = patches.iter().map(|patch| {
            let mut i = patch.split('=').into_iter();
            return (i.clone().nth(0), i.nth(1));
        });
        for key_values in split_with_equation {
            if let (Some(k), Some(v)) = key_values {
                let para = k.split('.').nth(1);
                if k.starts_with("challenge_linear") {
                    let p = para.ok_or_else(|| {
                        Error::PatchParameterError(
                            "parameters of challenge linear are absent".to_string(),
                        )
                    })?;
                    let mut w = self.challenge_linear.ok_or_else(|| {
                        Error::PatchParameterError("challenge linear absent".to_string())
                    })?;
                    w.apply_patch(p, v)?;
                    w.validate()?;
                    self.challenge_linear = Some(w);
                } else if k.starts_with("bond_linear") {
                    let p = para.ok_or_else(|| {
                        Error::PatchParameterError(
                            "parameters of bond linear are absent".to_string(),
                        )
                    })?;
                    let mut f = self.bond_linear.ok_or_else(|| {
                        Error::PatchParameterError("bond linear absent".to_string())
                    })?;
                    f.apply_patch(p, v)?;
                    f.validate()?;
                    self.bond_linear = Some(f);
                } else if k.starts_with("reward_split") {
                    let p = para.ok_or_else(|| {
                        Error::PatchParameterError(
                            "parameter of reward split parameter is absent".to_string(),
                        )
                    })?;
                    let mut f = self.reward_split.ok_or_else(|| {
                        Error::PatchParameterError("reward split config absent".to_string())
                    })?;
                    f.apply_patch(p, v)?;
                    f.validate()?;
                    self.reward_split = Some(f);
                } else if k.starts_with("reward_treasury_last") {
                    let p = para.ok_or_else(|| {
                        Error::PatchParameterError(
                            "parameter of reward treasury last model is absent".to_string(),
                        )
                    })?;
                    let mut f = self.reward_treasury_last.ok_or_else(|| {
                        Error::PatchParameterError("reward treasury last config absent".to_string())
                    })?;
                    f.apply_patch(p, v)?;
                    f.validate()?;
                    self.reward_treasury_last = Some(f);
                } else if k.starts_with("challenge_function") {
                    self.challenge_function = v.to_string();
                } else if k.starts_with("bond_function") {
                    self.bond_function = v.to_string();
                } else if k.starts_with("reward_function") {
                    self.reward_function = v.to_string();
                }
            } else {
                return Err(Error::PatchParameterError(
                    key_values.0.unwrap_or_default().to_string(),
                ));
            }
        }
        Ok(())
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
/// This config is used for Relayer or Challenger
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
        c.challenge_function.make_ascii_uppercase();
        if let Some(w) = c.challenge_linear {
            w.validate()?;
        }
        if let Some(f) = c.bond_linear {
            f.validate()?;
        }
        if let Some(r) = c.reward_split {
            r.validate()?;
        }

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
            // TODO: check name should be use number
            r.choice.make_ascii_uppercase();
            max_chose = std::cmp::max(max_chose, r.choice.len());
            for c in r.choice.chars() {
                if c != 'H' && c != 'L' && c != 'N' {
                    return Err(Error::ParameterError("relayer chose must be 'H', 'L', 'N'"));
                }
            }
        }
        if c.challengers.is_some() {
            // currently, we just handle the scenario with one challenger
            if c.challengers.clone().unwrap().len() > 1 || c.relayers.len() > 1 {
                return Err(Error::ParameterError(
                    "current we only support the scenario for one relayer and one challenger in relayer-challenger mode",
                ));
            }
            let relayer = c.relayers[0].clone();
            // currently, challenger is always honest
            for (i, r) in c.challengers.clone().unwrap().iter_mut().enumerate() {
                if r.name.is_none() {
                    r.name = Some(format!(" {}", i));
                };
                for (i, c) in r.choice.chars().enumerate() {
                    let chose_from_relayer = match relayer.choice.chars().nth(i) {
                        Some(c) => c,
                        None => {
                            return Err(Error::ParameterError(
                                "currently we are not support that challenger does not challenge to relayer",
                            ));
                        }
                    };
                    if c == '0' {
                        if chose_from_relayer != 'L' {
                            return Err(Error::ParameterError(
                                "currently we are not support challenger to lie",
                            ));
                        }
                    } else if c == '1' {
                        if chose_from_relayer != 'H' {
                            return Err(Error::ParameterError(
                                "currently we are not support challenger to lie",
                            ));
                        }
                    } else {
                        return Err(Error::ParameterError("challenger chose must be '0', '1'"));
                    }
                }
            }
        } else {
            let mut relayers = vec![RelayerConfig {
                name: Some("Darwinia".to_string()),
                choice: "H".repeat(max_chose + 1),
            }];
            relayers.append(&mut c.relayers);
            c.relayers = relayers;
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TOML_CONFIG: &'static str = r#"
			challenge_function = "linear"
			target_function = "half"
			bond_function = "10.0"
			reward_function = "split"

			Dd = 100
			De = 1000

			[challenge_linear]
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

        let bond_function = c.get_bond_equation();
        assert!(bond_function.is_ok());
        assert_eq!(bond_function.unwrap().calculate(0), 10.0);
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
			challenge_function = "linear"
			target_function = "half"
			bond_function = "10.0"
			reward_function = "split"

			[challenge_linear]
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
			challenge_function = "linear"
			target_function = "half"
			bond_function = "10.0"
			reward_function = "split"

			[challenge_linear]
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
    #[test]
    fn test_apply_patch() {
        let mut c = <ScenarioConfig>::from_str(TOML_CONFIG).unwrap();
        c.apply_patch(vec!["challenge_function=9487"]).unwrap();
        let challenge_function = c.get_challenge_equation();
        assert!(challenge_function.is_ok());
        assert_eq!(challenge_function.unwrap().calculate(10, 10), 9487);

        c.apply_patch(vec!["bond_function=1.2222"]).unwrap();
        let bond_function = c.get_bond_equation();
        assert!(bond_function.is_ok());
        assert_eq!(bond_function.unwrap().calculate(0), 1.2222);
    }
    #[test]
    fn test_() {
        let mut rp = RelayPositions::default();
        rp.relay_blocks.push(500);
        assert_eq!(
            rp.plot(),
            "G===============================================================1==>".to_string()
        );
        rp.relay_blocks.push(250);
        assert_eq!(
            rp.plot(),
            "G===============================2===============================1==>".to_string()
        );
        rp.relay_blocks.push(125);
        assert_eq!(
            rp.plot(),
            "G===============3===============2===============================1==>".to_string()
        );
        rp.relay_blocks.push(187);
        assert_eq!(
            rp.plot(),
            "G===============3======4========2===============================1==>".to_string()
        );
    }
    #[test]
    fn test_challenger_mod() {
        let c = <ScenarioConfig>::from_str(
            r#"
			challenge_function = "linear"
			target_function = "half"
			bond_function = "10.0"
			reward_function = "split"

			[challenge_linear]
			Wd = 0.0
			We = 0.0
			C  = 1
			Md = 100
			Me = 100
			B = 1
			T = 10

			[[relayers]]
			name = "Evil"
			choice = "LHLL"

			[[challengers]]
			name = "Challenger"
			choice = "0100"
			"#,
        );
        assert_eq!(c.is_ok(), true);
    }
    #[test]
    fn test_should_error_with_challengers() {
        let c = <ScenarioConfig>::from_str(
            r#"
			challenge_function = "linear"
			target_function = "half"
			bond_function = "10.0"
			reward_function = "split"

			[challenge_linear]
			Wd = 0.0
			We = 0.0
			C  = 1
			Md = 100
			Me = 100
			B = 1
			T = 10

			[[relayers]]
			name = "Evil"
			choice = "LHLL"

			[[challengers]]
			name = "Challenger1"
			choice = "0100"

			[[challengers]]
			name = "Challenger2"
			choice = "0100"
			"#,
        );
        assert_eq!(c.is_ok(), false);
    }
}
