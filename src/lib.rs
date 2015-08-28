/// We use this numerical type for all calculations.
pub type Num = f32;

/// Represents the state of a neuron.
#[derive(Copy, Clone)]
pub struct State {
    /// membrane potential of neuron (in mV)
    v: Num,

    /// recovery variable
    u: Num
}

/// At which potential the neuron's potential is reset to `c`.
const RESET_THRESHOLD : Num = 30.0;

impl State {
    pub fn new() -> State {
        State {
            v: -70.0,
            u: -14.0
        }
    }
    pub fn potential(&self) -> Num {
        if self.v < RESET_THRESHOLD { self.v } else { RESET_THRESHOLD }
    }

    pub fn recovery(&self) -> Num {
        self.u
    }
}

/// The neuron configuration parameters.
pub struct Config {
    /// Rate of recovery.
    a: Num,

    /// Sensitivity of recovery variable `u` to the membrane potential `v`.
    b: Num,

    /// After-spike reset value of membrane potential `v`.
    c: Num,

    /// After-spike reset of recovery variable `u`.
    d: Num
}

impl Config {
    /// Generates an excitatory neuron configuration according to Izhikevich's paper [reentry]
    /// where `r` is a random variable uniformly distributed in [0, 1].
    pub fn excitatory(r: Num) -> Config {
        debug_assert!(r >= 0.0 && r <= 1.0);

        let r2 = r*r;
        Config {
            a: 0.02,
            b: 0.2,
            c: -65.0 + 15.0 * r2,
            d: 8.0 - 6.0 * r2
        }
    }

    pub fn inhibitory(r: Num) -> Config {
        debug_assert!(r >= 0.0 && r <= 1.0);

        Config {
            a: 0.02 + 0.08 * r,
            b: 0.25 - 0.05 * r,
            c: -65.0,
            d: 2.0
        }
    }

    /// Regular spiking (RS) cell configuration.
    pub fn regular_spiking() -> Config {
        Config::excitatory(0.0)
    }

    /// Chattering (CH) cell configuration.
    pub fn chattering() -> Config {
        Config::excitatory(1.0)
    }
}

#[inline(always)]
fn dv(u: Num, v: Num, i_syn: Num) -> Num {
    (0.04 * v + 5.0) * v + 140.0 - u + i_syn
}

#[inline(always)]
fn du(u: Num, v: Num, a: Num, b: Num) -> Num {
    a * (b*v - u)
}

impl State {
    #[inline(always)]
    fn calc(self, dt: Num, i_syn: Num, config: &Config) -> State {
        State {
            v: self.v + dt*dv(self.u, self.v, i_syn),
            u: self.u + dt*du(self.u, self.v, config.a, config.b)
        }
    }

    /// Calculate the new state after 1 ms.
    #[inline(always)]
    pub fn step_1ms(self, i_syn: Num, config: &Config) -> (State, bool) {
        if self.v < RESET_THRESHOLD {
            (self.calc(0.5, i_syn, config).calc(0.5, i_syn, config), false)
        }
        else {
            (State {
                v: config.c,
                u: self.u + config.d
            }, true)
        }
    }
}
