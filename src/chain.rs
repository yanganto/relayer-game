///! Simulation chain
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::scenario::{RelayerConfig, ScenarioConfig};
use crate::target::Equation;

static TOTAL_RELAYER: AtomicUsize = AtomicUsize::new(0);

#[derive(Default, Debug)]
pub struct ChainsStatus {
    pub darwinia_block_hight: usize,
    pub ethereum_block_hight: usize,
    /// Last relayed block info
    /// (darwinia_block_height_for_last_relay, ethereum_block_height_for_last_relay)
    pub relayers: HashMap<String, RelayerStatus>,
    pub submit_target_ethereum_block: usize,
    pub submitions: Vec<(usize, usize)>,
    pub block_speed_factor: f64,
    pub submit_bond_pool: f64,
}

impl From<ScenarioConfig> for ChainsStatus {
    fn from(c: ScenarioConfig) -> Self {
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
        write!(f, "{}{}", self.fmt_status(), self.fmt_relayers_status())
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

    pub fn should_balance(&self) {
        let mut p = self.submit_bond_pool;
        for (_key, r) in self.relayers.iter() {
            p -= r.pay;
            p += r.reward;
        }

        // TODO: check the small number is correct and acceptable
        if p != 0.0 || p > 0.00000001 {
            println!("p: {}", p);
            panic!("System unbalance");
        }
    }

    pub fn reward_honest_relayers(&mut self) {
        let total_honest_submit_times = self.relayers.iter().fold(0, |mut sum, (_k, r)| {
            sum += r.get_honest_submit_times();
            sum
        });
        let share_pre_submit = self.submit_bond_pool / total_honest_submit_times as f64;
        for r in self.relayers.values_mut() {
            r.reward += r.get_honest_submit_times() as f64 * share_pre_submit;
        }
        self.submit_bond_pool = 0.0;
    }
}

#[derive(Default, Debug)]
pub struct RelayerStatus {
    pub id: usize,
    pub name: Option<String>,
    pub pay: f64,
    pub reward: f64,
    pub submit_times: usize,
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
        let balance = self.reward - self.pay;
        if let Some(n) = &self.name {
            write!(f, "{}: {} ", n, balance)
        } else {
            write!(f, "ID {}: {} ", self.id, balance)
        }
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
        c.reward_honest_relayers();
        c.should_balance();
        assert_eq!(c.relayers["Evil"].reward, 0.0);
        assert_eq!(c.relayers["Darwinia"].reward, 30.0);
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
        c.reward_honest_relayers();
        c.should_balance();
        assert_eq!(c.relayers["Evil"].reward, 0.0);
        assert_eq!(c.relayers["Darwinia"].reward, 30.0);
    }
}
