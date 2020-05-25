///! Simulation chain
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::sample::Equation;
use crate::scenario::{RelayerConfig, ScenarioConfig};

static TOTAL_RELAYER: AtomicUsize = AtomicUsize::new(0);
static VISUALIZED_MAX_LENGTH: usize = 64;

/// # RewardFrom
/// The source of reward can from the slash value of evil relayer, or the trasury.
/// In somecase, there is no evil relayer in the submitround, so the relayer may be reward by
/// treasury.  However, in production, the treasury reward part may be some points.
/// The user will pay a fee to treasury in redeem action, and then the relayer get the
/// reward from the fee accorance with the share of powint
///
#[derive(Debug)]
pub enum RewardFrom {
    Treasure,
    Slash,
}

/// # Reward
/// This is the action structure for pay reward to someone
#[derive(Debug)]
pub struct Reward {
    /// The reward value from slash or treasury
    pub from: RewardFrom,
    /// To the user (relayer or challenger)
    pub to: String,
    /// The value should reward
    pub value: f64,
}

/// # Chains Status
/// simulate  the status both Darwinia and Ethereum
/// The status of challenger and relauer are the same,
/// the only different is their behavior
#[derive(Default, Debug)]
pub struct ChainsStatus {
    /// The current block height of Darwinia
    pub darwinia_block_hight: usize,
    /// The current block height of Ethereum
    pub ethereum_block_hight: usize,
    /// The relayer status
    pub relayers: HashMap<String, RelayerStatus>,
    /// The challenger status
    pub challengers: HashMap<String, RelayerStatus>,
    /// The next Ethereum block that relayer should submit
    pub submit_target_ethereum_block: usize,
    /// The list of submission
    pub submitions: Vec<(usize, usize)>,
    /// The factor for the block producing speed
    pub block_speed_factor: f64,
    /// The pool to store the bond value from relayer or challenger
    pub submit_bond_pool: f64,
    /// We do not simulate the redeem action and the fee, so the debt will occure when pay from
    /// treasury
    pub treasury_debt: f64,
}

impl From<ScenarioConfig> for ChainsStatus {
    fn from(c: ScenarioConfig) -> Self {
        let challengers = if let Some(cs) = c.challengers.clone() {
            cs
        } else {
            vec![]
        };
        ChainsStatus {
            darwinia_block_hight: c.Dd.unwrap_or(0),
            ethereum_block_hight: c.De.unwrap_or(100),
            relayers: c
                .relayers
                .clone()
                .into_iter()
                .fold(HashMap::new(), |mut map, r| {
                    let s: RelayerStatus = r.into();
                    if let Some(n) = &s.name {
                        map.insert(n.to_string(), s);
                    } else {
                        map.insert(format!(" {}", s.id), s);
                    }
                    map
                }),
            challengers: challengers.into_iter().fold(HashMap::new(), |mut map, r| {
                let s: RelayerStatus = r.into();
                if let Some(n) = &s.name {
                    map.insert(n.to_string(), s);
                } else {
                    map.insert(format!(" {}", s.id), s);
                }
                map
            }),
            submit_target_ethereum_block: c
                .get_target_equation()
                .unwrap()
                .calculate(0, c.De.unwrap_or(100)),
            block_speed_factor: c.F.unwrap_or(2.0),
            ..Default::default()
        }
    }
}

impl fmt::Display for ChainsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.fmt_status(),
            self.fmt_relayers_status(),
            self.fmt_challengers_status()
        )
    }
}

impl ChainsStatus {
    pub fn fmt_status(&self) -> String {
        let submition_times = self.submitions.len();
        let last_relayed_block = if submition_times > 0 {
            self.submitions[submition_times - 1]
        } else {
            (0, 0)
        };
        format!(
            "ChainsStatus: Darwinia #{}, Ethereum #{}, Submit at Eth(#{}) Last relay Eth(#{}) at #{}\n",
            self.darwinia_block_hight,
            self.ethereum_block_hight,
			self.submit_target_ethereum_block,
            last_relayed_block.1,
            last_relayed_block.0
        )
    }
    pub fn fmt_relayers_status(&self) -> String {
        let mut output = String::new();
        for r in self.relayers.iter() {
            output.push_str(&format!("{}", r.1));
        }
        output
    }
    pub fn fmt_challengers_status(&self) -> String {
        let mut output = String::new();
        for r in self.challengers.iter() {
            output.push_str(&format!("{}", r.1));
        }
        output
    }
    pub fn fmt_relayers_bar_chart(&self, normalize_value: f64) -> String {
        let mut output = String::new();
        for r in self.relayers.iter() {
            output.push_str(&r.1.format_to_bar_char(normalize_value, VISUALIZED_MAX_LENGTH));
        }
        for c in self.challengers.iter() {
            output.push_str(&c.1.format_to_bar_char(normalize_value, VISUALIZED_MAX_LENGTH));
        }
        output.push_str("-: slash, +: reward from slash, *: reward from treasury");
        output
    }
    fn submit_by(&mut self, relayer: String, bond: f64, lie: bool) {
        let r = self.relayers.get_mut(&relayer).unwrap();
        r.submit(bond, lie);
        self.submit_bond_pool += bond;
    }
    pub fn submit(
        &mut self,
        relayers: Vec<(String, bool)>,
        bond: f64,
        wait_blocks: usize,
        next_target_ethereum_block: usize,
    ) {
        for (relayer, lie) in relayers {
            self.submit_by(relayer, bond, lie)
        }
        self.submitions
            .push((self.darwinia_block_hight, self.submit_target_ethereum_block));
        self.ethereum_block_hight += (wait_blocks as f64 / self.block_speed_factor) as usize;
        self.darwinia_block_hight += wait_blocks;
        self.submit_target_ethereum_block = next_target_ethereum_block;
    }
    pub fn challenge_by(&mut self, challenger: String, bond: f64) {
        let challenger = self.challengers.get_mut(&challenger).unwrap();
        challenger.pay += bond;
        self.submit_bond_pool += bond;
    }

    pub fn should_balance(&self) {
        let mut p = self.submit_bond_pool - self.treasury_debt;
        for (_key, r) in self.relayers.iter() {
            p -= r.pay;
            p += r.reward();
        }
        for (_key, c) in self.challengers.iter() {
            p -= c.pay;
            p += c.reward();
        }

        // TODO: check the small number is correct and acceptable
        if p != 0.0 && p > 0.00000001 {
            println!("p: {}", p);
            println!("Chain Status: {:?}", self);
            panic!("System unbalance");
        }
    }
    pub fn reward(&mut self, rewards: Vec<Reward>) {
        for reward in rewards.into_iter() {
            let mut reciver = self.relayers.get_mut(&reward.to);
            if reciver.is_none() {
                reciver = self.challengers.get_mut(&reward.to);
            };
            let r = reciver.unwrap();
            if !r.lie {
                match reward.from {
                    RewardFrom::Treasure => {
                        r.reward.1 += reward.value;
                        self.treasury_debt += reward.value;
                    }
                    RewardFrom::Slash => {
                        r.reward.0 += reward.value;
                        self.submit_bond_pool -= reward.value;
                    }
                }
            }
        }
    }
}

/// # RelayerStatus
/// the statue we simulate
#[derive(Default, Debug, Clone)]
pub struct RelayerStatus {
    /// id is used when name not provided
    pub id: usize,
    /// name is option field in scenario file
    pub name: Option<String>,
    /// current pay out for bond
    pub pay: f64,
    /// the reward from slash (reward.0) and from treasury (reward.1)
    pub reward: (f64, f64),
    /// the total times the relayer submit
    pub submit_times: usize,
    /// the relayer has lied or not
    pub lie: bool,
}

impl From<RelayerConfig> for RelayerStatus {
    fn from(c: RelayerConfig) -> Self {
        RelayerStatus {
            id: TOTAL_RELAYER.fetch_add(1, Ordering::SeqCst),
            name: c.name,
            ..Default::default()
        }
    }
}

impl fmt::Display for RelayerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let balance = self.reward() - self.pay;
        if let Some(n) = &self.name {
            write!(f, "{}({}): {} ", n, self.get_honest_submit_times(), balance)
        } else {
            write!(
                f,
                "ID {}({}): {} ",
                self.id,
                self.get_honest_submit_times(),
                balance
            )
        }
    }
}

impl RelayerStatus {
    fn reward(&self) -> f64 {
        self.reward.0 + self.reward.1
    }
    fn format_to_bar_char(&self, normalize_value: f64, normalize_width: usize) -> String {
        let reward_slash_part: usize;
        let reward_treasury_part: usize;
        let slash_part: usize;
        if self.reward.0 > self.pay {
            reward_slash_part =
                ((self.reward.0 - self.pay) / normalize_value * normalize_width as f64) as usize;
            reward_treasury_part =
                (self.reward.1 / normalize_value * normalize_width as f64) as usize;

            slash_part = 0;
        } else {
            reward_slash_part = 0;
            reward_treasury_part = 0;
            slash_part = (self.pay / normalize_value * normalize_width as f64) as usize;
        }
        format!(
            " {}{}{} {}\n",
            "-".repeat(slash_part).to_string(),
            "+".repeat(reward_slash_part),
            "*".repeat(reward_treasury_part),
            self
        )
    }
}

impl RelayerStatus {
    fn submit(&mut self, bond: f64, lie: bool) {
        self.pay += bond;
        self.lie |= lie;
        self.submit_times += 1;
    }

    fn get_honest_submit_times(&self) -> usize {
        if self.lie {
            0
        } else {
            self.submit_times
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
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

			[reward_split]
			P = 0.5

			[[relayers]]
			name = "Evil"
			choice = "L"
			"#;
    #[test]
    fn test_chain_status_from_scenario_config_with_submit_by_replysers() {
        let mut c: ChainsStatus = <ScenarioConfig>::from_str(TOML_CONFIG).unwrap().into();
        assert_eq!(c.relayers["Darwinia"].lie, false);
        c.submit_by("Evil".to_string(), 10.0, true); // `true` is to lie
        c.should_balance();
        c.submit_by("Darwinia".to_string(), 10.0, false);
        c.should_balance();
        c.submit_by("Darwinia".to_string(), 10.0, false);
        c.should_balance();
        assert_eq!(c.submit_bond_pool, 30.0);
        c.reward(vec![Reward {
            from: RewardFrom::Slash,
            to: "Darwinia".to_string(),
            value: 30.0,
        }]);
        c.should_balance();
        assert_eq!(c.relayers["Evil"].reward(), 0.0);
        assert_eq!(c.relayers["Darwinia"].reward(), 30.0);
    }
    #[test]
    fn test_chain_status_from_scenario_config() {
        let mut c: ChainsStatus = <ScenarioConfig>::from_str(TOML_CONFIG).unwrap().into();
        assert_eq!(c.relayers["Darwinia"].lie, false);
        c.submit(
            vec![("Evil".to_string(), true), ("Darwinia".to_string(), false)],
            10.0,
            50,
            500,
        );
        c.should_balance();
        c.submit(vec![("Darwinia".to_string(), false)], 10.0, 50, 250);
        c.should_balance();
        assert_eq!(c.submit_bond_pool, 30.0);
        c.reward(vec![Reward {
            from: RewardFrom::Slash,
            to: "Darwinia".to_string(),
            value: 30.0,
        }]);
        c.should_balance();
        assert_eq!(c.relayers["Evil"].reward(), 0.0);
        assert_eq!(c.relayers["Darwinia"].reward(), 30.0);
    }
}
