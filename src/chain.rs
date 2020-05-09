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
    last_relayed_block: (usize, usize),
    relayers: HashMap<String, RelayerStatus>,
    submit_times: usize,
    submit_target: usize,
    block_speed_factor: f32,
    submit_fee_pool: usize,
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
            self.last_relayed_block.0,
            self.last_relayed_block.1
        );
        for r in self.relayers.iter() {
            output.push_str(&format!("{}", r.1));
        }
        write!(f, "{}", output)
    }
}

#[derive(Default)]
pub struct RelayerStatus {
    pub id: usize,
    pub name: Option<String>,
    pub pay: usize,
    pub reward: usize,
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
