use crate::network::NeuronId;
use crate::simulation::{EventRecorder, Timestep};

#[derive(Debug)]
pub struct DummyRecorder;

impl EventRecorder for DummyRecorder {
    fn record_fire(&mut self, _neuron_id: NeuronId, _time_step: Timestep) {}
}
