use crate::model::{NeuronActivity, NeuronConfig, NeuronState, StdpConfig};
use crate::network::SynapseId;
use crate::Num;

#[derive(Debug)]
pub struct Neuron {
    // internal neuron state
    pub(crate) state: NeuronState,

    // external neuron state
    pub(crate) i_ext: Num,
    pub(crate) i_inp: Num,

    // learning

    // XXX: Have a list of `active` synapses, which are active
    // once a neuron fires on it. The `active` synapses track
    // themselves, once they become inactive. Use a similar
    // `decay` process as for STDP for the action potential,
    // so that the shape of the spike is less sharp. A high
    // decay rate can simulate the old behaviour.
    //
    // Spike-Time Dependent Plasticity
    //
    // when a neuron fires, we set this value to STDP_FIRE_RESET
    // (0.1 for example), and during every time-step we decay
    // it by STDP_DECAY e.g. 0.95.
    pub(crate) stdp: Num,

    // Neuron model parameters
    pub(crate) config: NeuronConfig,

    // connectivity
    pub(crate) pre_synapses: Vec<SynapseId>,
    pub(crate) post_synapses: Vec<SynapseId>,
}

impl Neuron {
    // Update the internal neuron state according to the synaptic input.
    // Move into simulator
    pub fn update_state(&mut self, stdp_config: StdpConfig) -> NeuronActivity {
        // synaptic input
        let syn_i = self.i_ext + self.i_inp;

        let (new_state, activity) = self.state.step_1ms(syn_i, &self.config);
        self.state = new_state;

        match activity {
            NeuronActivity::Fires => {
                // Reset the neurons STDP to a high value.
                self.stdp = stdp_config.fire_reset;
            }
            NeuronActivity::Silent => {
                // decay STDP
                self.stdp *= stdp_config.decay;
            }
        }

        return activity;
    }
}
