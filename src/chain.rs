use std::collections::HashMap;
///! Simulation chain
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::scenario::{RelayerConfig, ScenarioConfig};

static TOTAL_RELAYER: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
pub struct ChainsStatus {
    darwinia_block_hight: usize,
    ethereum_block_hight: usize,
    /// Last relayed block info
    /// (darwinia_block_height_for_last_relay, ethereum_block_height_for_last_relay)
    last_relayed_block: (usize, usize),
    relayers: HashMap<String, RelayerStatus>,
    submit_target: usize,
    block_speed_factor: f32,
    submit_fee_pool: f32,
}

impl From<ScenarioConfig> for ChainsStatus {
    fn from(c: ScenarioConfig) -> Self {
        ChainsStatus {
            darwinia_block_hight: c.Dd.unwrap_or(0),
            ethereum_block_hight: c.De.unwrap_or(100),
            last_relayed_block: (0, 0),
            relayers: c.relayers.into_iter().fold(HashMap::new(), |mut map, r| {
                let s: RelayerStatus = r.into();
                if let Some(n) = &s.name {
                    map.insert(n.to_string(), s);
                } else {
                    map.insert(format!(" {}", s.id), s);
                }
                map
            }),
            submit_target: c.De.unwrap_or(100) / 2, // TODO: make submit target as a function
            block_speed_factor: c.F.unwrap_or(2.0),
            ..Default::default()
        }
    }
}

impl fmt::Display for ChainsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!(
            "ChainsStatus: Darwinia #{}, Ethereum #{}, Last relay Eth(#{}) at #{}\n",
            self.darwinia_block_hight,
            self.ethereum_block_hight,
            self.last_relayed_block.1,
            self.last_relayed_block.0
        );
        for r in self.relayers.iter() {
            output.push_str(&format!("{}", r.1));
        }
        write!(f, "{}", output)
    }
}

impl ChainsStatus {
    fn submit(&mut self, relayer: String, fee: f32, lie: bool) {
        let r = self.relayers.get_mut(&relayer).unwrap();
        r.submit(fee, lie);
        self.submit_fee_pool += fee;
    }

    fn should_balance(&self) {
        let mut p = self.submit_fee_pool;
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

    fn reward_honest_relayers(&mut self) {
        let total_honest_submit_times = self.relayers.iter().fold(0, |mut sum, (_k, r)| {
            sum += r.get_honest_submit_times();
            sum
        });
        let share_pre_submit = self.submit_fee_pool / total_honest_submit_times as f32;
        for r in self.relayers.values_mut() {
            r.reward += r.get_honest_submit_times() as f32 * share_pre_submit;
            r.lie = false;
            r.submit_times = 0;
        }
        self.submit_fee_pool = 0.0;
    }
}

#[derive(Default)]
pub struct RelayerStatus {
    pub id: usize,
    pub name: Option<String>,
    pub pay: f32,
    pub reward: f32,
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
            write!(f, "  Relayer {}: {}", n, balance)
        } else {
            write!(f, "  Relayer ID {}: {}", self.id, balance)
        }
    }
}

impl RelayerStatus {
    fn submit(&mut self, fee: f32, lie: bool) {
        self.pay += fee;
        self.lie &= lie;
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
			wait_function = "linear"
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
			name = "evil"
			choice = "FFFFFF" 
		"#;
    #[test]
    fn test_chain_status_from_scenario_config() {
        let mut c: ChainsStatus = <ScenarioConfig>::from_str(TOML_CONFIG).unwrap().into();
        assert_eq!(c.relayers["Darwinia"].lie, false);
        c.submit("evil".to_string(), 10.0, true); // `true` is to lie
        c.should_balance();
        c.submit("Darwinia".to_string(), 10.0, false);
        c.should_balance();
        c.reward_honest_relayers();
        c.should_balance();
    }
}
