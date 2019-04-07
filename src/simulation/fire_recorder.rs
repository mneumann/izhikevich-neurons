use crate::network::NeuronId;
use crate::simulation::{EventRecorder, Timestep};

#[derive(Debug)]
pub struct FireRecorder {
    pub events: Vec<(NeuronId, Timestep)>,
}

impl FireRecorder {
    pub fn new() -> FireRecorder {
        FireRecorder { events: Vec::new() }
    }
}

impl EventRecorder for FireRecorder {
    fn record_fire(&mut self, neuron_id: NeuronId, time_step: Timestep) {
        self.events.push((neuron_id, time_step));
    }
}
