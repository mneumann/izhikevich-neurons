use crate::network::NeuronId;
use crate::simulation::Timestep;

pub trait EventRecorder {
    fn record_fire(&mut self, neuron_id: NeuronId, time_step: Timestep);
}
