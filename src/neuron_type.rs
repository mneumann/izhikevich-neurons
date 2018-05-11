use Num;
use closed01::Closed01;
use neuron_config::NeuronConfig;

/// Datastructure representing one of the broad neuron type.
/// Can be directly converted into a NeuronConfig.
pub enum NeuronType {
    Excitatory(Closed01<Num>),
    Inhibitory(Closed01<Num>),
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
