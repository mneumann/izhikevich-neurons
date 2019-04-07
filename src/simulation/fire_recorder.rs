use crate::network::NeuronId;
use crate::simulation::Timestep;

#[derive(Debug)]
pub struct FireRecorder {
    pub events: Vec<(NeuronId, Timestep)>,
}

impl FireRecorder {
    pub fn new() -> FireRecorder {
        FireRecorder { events: Vec::new() }
    }

    pub fn record(&mut self, neuron_id: NeuronId, time_step: Timestep) {
        self.events.push((neuron_id, time_step));
    }
}
