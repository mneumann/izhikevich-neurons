use crate::model::NeuronConfig;
use crate::Closed01;

/// Datastructure representing one of the broad neuron types.
/// Can be directly converted into a NeuronConfig.
pub enum NeuronType {
    Excitatory(Closed01),
    Inhibitory(Closed01),
    RegularSpiking,
    Chattering,
}

impl Into<NeuronConfig> for NeuronType {
    fn into(self) -> NeuronConfig {
        match self {
            NeuronType::Excitatory(r) => NeuronConfig::excitatory(r),
            NeuronType::Inhibitory(r) => NeuronConfig::inhibitory(r),
            NeuronType::RegularSpiking => NeuronConfig::regular_spiking(),
            NeuronType::Chattering => NeuronConfig::chattering(),
        }
    }
}
