/// Describes the activity of a neuron after one step of simulation.
/// A neuron either fires an action potential or it is silent.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NeuronActivity {
    /// The neuron fires an action potential
    Fires,
    /// The neuron is silent.
    Silent,
}

impl NeuronActivity {
    pub fn fires(&self) -> bool {
        *self == NeuronActivity::Fires
    }
}
