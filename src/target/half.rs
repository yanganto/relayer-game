use crate::target::Equation;

/// There is no parameter for Half Target Equation
pub struct HalfConfig {}

impl Equation for HalfConfig {
    /// target block = (relayed_header - submit_header) / 2
    fn calculate(&self, relayed_header: usize, submit_header: usize) -> usize {
        (submit_header - relayed_header) / 2
    }
}
