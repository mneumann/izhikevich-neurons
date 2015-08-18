type Num = f32;

/// Represents the state of a neuron.
struct State {
    /// membrane potential of neuron (in mV)
    v: Num,

    /// recovery variable
    u: Num
}

/// The neuron configuration parameters.
struct Config {

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
    fn excitatory(r: Num) -> Config {
        debug_assert!(r >= 0.0 && r <= 1.0);

        let r2 = r*r;
        Config {
            a: 0.02,
            b: 0.2,
            c: -65.0 + 15.0 * r2,
            d: 8.0 - 6.0 * r2
        }
    }

    fn inhibitory(r: Num) -> Config {
        debug_assert!(r >= 0.0 && r <= 1.0);

        Config {
            a: 0.02 + 0.08 * r,
            b: 0.25 - 0.05 * r,
            c: -65.0,
            d: 2.0
        }
    }

    /// Regular spiking (RS) cell configuration.
    fn regular_spiking() -> Config {
        Config::excitatory(0.0)
    }

    /// Chattering (CH) cell configuration.
    fn chattering() -> Config {
        Config::excitatory(1.0)
    }
}

#[inline(always)]
fn dv(u: Num, v: Num, i_syn: Num) -> Num {
    0.04 * (v*v) + 5.0 * v + 140.0 - u - i_syn
}

#[inline(always)]
fn du(u: Num, v: Num, a: Num, b: Num) -> Num {
    a * (b*v - u)
}

impl State {
    /// Calculate the state after `dt` ms.
    /// If second return parameter is true, then the neuron fired.
    fn step(self, dt: Num, i_syn: Num, config: &Config) -> (State, bool) {
        let next = State {
            v: self.v + dt*dv(self.u, self.v, i_syn),
            u: self.u + dt*du(self.u, self.v, config.a, config.b)
        };
        if next.v >= 30.0 {
            (State { v: config.c, u: next.u + config.d }, true)
        } else {
            (next, false)
        }
    }
}

fn main() {
}
