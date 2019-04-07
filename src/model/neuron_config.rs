use crate::Num;
use closed01::Closed01;

/// A neuron's configuration parameters.
#[derive(Debug)]
pub struct NeuronConfig {
    /// Rate of recovery.
    pub(crate) a: Num,

    /// Sensitivity of recovery variable `u` to the membrane potential `v`.
    pub b: Num,

    /// After-spike reset value of membrane potential `v`.
    pub c: Num,

    /// After-spike reset of recovery variable `u`.
    pub d: Num,
}

impl NeuronConfig {
    /// Generates an excitatory neuron configuration according to Izhikevich's paper [reentry]
    /// where `r` is a random variable uniformly distributed in [0, 1].
    pub fn excitatory(r: Closed01<Num>) -> NeuronConfig {
        let r = r.get();
        let r2 = r * r;
        NeuronConfig {
            a: 0.02,
            b: 0.2,
            c: -65.0 + 15.0 * r2,
            d: 8.0 - 6.0 * r2,
        }
    }

    pub fn inhibitory(r: Closed01<Num>) -> NeuronConfig {
        let r = r.get();
        NeuronConfig {
            a: 0.02 + 0.08 * r,
            b: 0.25 - 0.05 * r,
            c: -65.0,
            d: 2.0,
        }
    }

    /// Regular spiking (RS) cell configuration.
    pub fn regular_spiking() -> NeuronConfig {
        NeuronConfig::excitatory(Closed01::new(0.0))
    }

    /// Chattering (CH) cell configuration.
    pub fn chattering() -> NeuronConfig {
        NeuronConfig::excitatory(Closed01::new(1.0))
    }
}
