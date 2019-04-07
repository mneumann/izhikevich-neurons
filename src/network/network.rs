use crate::model::NeuronState;
use crate::network::{Neuron, NeuronId, Synapse, SynapseDelay, SynapseId};
use crate::Num;

#[derive(Debug)]
pub struct Network {
    pub(crate) neurons: Vec<Neuron>,
    pub(crate) synapses: Vec<Synapse>,
}

impl Network {
    pub fn neuron_ref(&self, neuron_id: NeuronId) -> &Neuron {
        &self.neurons[neuron_id.index()]
    }

    pub fn neuron_mut(&mut self, neuron_id: NeuronId) -> &mut Neuron {
        &mut self.neurons[neuron_id.index()]
    }

    pub fn save_state(&self) -> Vec<NeuronState> {
        self.neurons
            .iter()
            .enumerate()
            .map(|(_, n)| n.state)
            .collect()
    }

    pub fn total_neurons(&self) -> usize {
        self.neurons.len()
    }

    /// Reset the input currents of all neurons
    pub fn reset_all_input_currents(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.i_inp = 0.0;
        }
    }

    /// Excite `neuron_id` with `current`.
    pub fn set_external_input(&mut self, neuron_id: NeuronId, current: Num) {
        self.neuron_mut(neuron_id).i_ext = current;
    }

    pub fn get_external_input(&self, neuron_id: NeuronId) -> Num {
        self.neuron_ref(neuron_id).i_ext
    }

    pub fn increase_external_input(&mut self, neuron_id: NeuronId, additional_current: Num) {
        self.neuron_mut(neuron_id).i_ext += additional_current;
    }

    pub fn process_firing_synapse(&mut self, firing_synapse: SynapseId) {
        let syn = &mut self.synapses[firing_synapse.index()];

        let pre_neuron_stdp = self.neurons[syn.pre_neuron.index()].stdp;
        let post_neuron = &mut self.neurons[syn.post_neuron.index()];
        let post_neuron_stdp = post_neuron.stdp;

        post_neuron.i_inp += syn.weight;

        // whenever a spike arrives here at it's post_neuron, this means, that
        // the pre-neuron fired some time ago (delay time-steps). It can be the
        // case that the post_neuron has fired ealier, in which case we have to
        // depress the synapse according to the STDP rule.
        syn.eff_d += pre_neuron_stdp - post_neuron_stdp;
    }

    /// The synapses `firing_synapses` fire. Update the network state.
    pub fn process_firing_synapses(&mut self, firing_synapses: &[SynapseId]) {
        for &syn_id in firing_synapses {
            self.process_firing_synapse(syn_id);
        }
    }

    // Excite the synapses that might have led to the firing of the underlying neuron.
    // We do this by adding the synapses pre_neuron's STDP value to the synapses eff_d
    // (efficacy derivative) value.
    //
    // We do not update the synapses weight value immediatly, but only once very while
    // (TODO), so that STDP reflects more LTP (Long Term Potentiation).
    pub fn excite_all_pre_synapses_of_neuron(&mut self, neuron_id: NeuronId) {
        for &synapse_id in self.neurons[neuron_id.index()].pre_synapses.iter() {
            let synapse = &mut self.synapses[synapse_id.index()];
            let stdp = self.neurons[synapse.pre_neuron.index()].stdp;
            synapse.eff_d += stdp;
        }
    }

    pub fn update_synapse_weights(
        &mut self,
        // range
        min_syn_weight: Num,
        max_syn_weight: Num,
        eff_d_decay: Num,
    ) {
        for syn in self.synapses.iter_mut() {
            let new_weight = syn.weight + syn.eff_d;

            // Restrict synapse weight min_syn_weight .. max_syn_weight
            syn.weight = clamp(new_weight, min_syn_weight, max_syn_weight);
            syn.eff_d *= eff_d_decay; // decay
        }
    }

    pub fn max_synapse_delay(&self) -> SynapseDelay {
        self.synapses
            .iter()
            .map(|syn| syn.synapse_delay)
            .max()
            .unwrap()
    }
}

fn clamp(value: Num, min: Num, max: Num) -> Num {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
