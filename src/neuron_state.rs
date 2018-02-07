use super::{NeuronConfig, Num};

/// At which potential the neuron's potential is reset to `c`.
const RESET_THRESHOLD: Num = 30.0;

/// Represents the state of a neuron.
#[derive(Copy, Clone, Debug)]
pub struct NeuronState {
    /// membrane potential of neuron (in mV)
    v: Num,

    /// recovery variable
    u: Num,
}

#[inline(always)]
fn dv(u: Num, v: Num, i_syn: Num) -> Num {
    (0.04 * v + 5.0) * v + 140.0 - u + i_syn
}

#[inline(always)]
fn du(u: Num, v: Num, a: Num, b: Num) -> Num {
    a * (b * v - u)
}

impl NeuronState {
    pub fn new() -> NeuronState {
        NeuronState { v: -70.0, u: -14.0 }
    }

    pub fn potential(&self) -> Num {
        if self.v < RESET_THRESHOLD {
            self.v
        } else {
            RESET_THRESHOLD
        }
    }

    pub fn recovery(&self) -> Num {
        self.u
    }

    #[inline(always)]
    fn calc(self, dt: Num, i_syn: Num, config: &NeuronConfig) -> NeuronState {
        NeuronState {
            v: self.v + dt * dv(self.u, self.v, i_syn),
            u: self.u + dt * du(self.u, self.v, config.a, config.b),
        }
    }

    /// Calculate the new state after 1 ms.
    #[inline]
    pub fn step_1ms(self, i_syn: Num, config: &NeuronConfig) -> (NeuronState, bool) {
        if self.v < RESET_THRESHOLD {
            (
                // Split into two half-steps (0.5ms) to improve numerical stability
                self.calc(0.5, i_syn, config).calc(0.5, i_syn, config),
                false, // Neuron does not fire
            )
        } else {
            (
                NeuronState {
                    v: config.c,
                    u: self.u + config.d,
                },
                true, // Neuron fires
            )
        }
    }
}
