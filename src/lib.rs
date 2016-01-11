extern crate closed01;
pub use neuron_state::NeuronState;
pub use neuron_config::NeuronConfig;
pub use neuron_type::NeuronType;
pub use network::Network;
pub use simulator::Simulator;

pub mod neuron_state;
pub mod neuron_config;
pub mod neuron_type;
pub mod network;
pub mod simulator;

/// We use this numerical type for all calculations.
pub type Num = f32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NeuronId(pub u32);

impl NeuronId {
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for NeuronId {
    #[inline(always)]
    fn from(index: usize) -> Self {
        NeuronId(index as u32)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynapseId(pub u32);

impl SynapseId {
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for SynapseId {
    #[inline(always)]
    fn from(index: usize) -> Self {
        SynapseId(index as u32)
    }
}

pub type TimeStep = u32;
pub type Delay = u8;

#[derive(Debug)]
pub struct FireRecorder {
    pub events: Vec<(NeuronId, TimeStep)>,
}

impl FireRecorder {
    pub fn new() -> FireRecorder {
        FireRecorder { events: Vec::new() }
    }

    pub fn record(&mut self, neuron_id: NeuronId, time_step: TimeStep) {
        self.events.push((neuron_id, time_step));
    }
}
