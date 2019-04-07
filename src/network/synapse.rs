use crate::network::{NeuronId, SynapseDelay};
use crate::Num;

#[derive(Debug)]
pub struct Synapse {
    // these are all static parameters
    pub(crate) pre_neuron: NeuronId,
    pub(crate) post_neuron: NeuronId,
    pub(crate) synapse_delay: SynapseDelay,
    pub(crate) weight: Num,

    // efficiacy derivative used for STDP
    pub(crate) eff_d: Num, // ... learning parameters
}
