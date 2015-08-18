extern crate gnuplot;

type Num = f32;

/// Represents the state of a neuron.
#[derive(Copy, Clone)]
struct State {
    /// membrane potential of neuron (in mV)
    v: Num,

    /// recovery variable
    u: Num
}

impl State {
    fn new() -> State {
        State {
            v: -70.0,
            u: -14.0
        }
    }
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
    /// Calculate the new state after `dt` ms.
    #[inline(always)]
    fn step(self, dt: Num, i_syn: Num, config: &Config) -> State {
        if self.v >= 30.0 {
            State {
                v: config.c,
                u: self.u + config.d
            }
        }
        else {
            State {
                v: self.v + dt*dv(self.u, self.v, i_syn),
                u: self.u + dt*du(self.u, self.v, config.a, config.b)
            }
        }
    }
}

fn main() {
    use gnuplot::{Figure, Caption, Color, AxesCommon};

    let config = Config::regular_spiking();
    let mut neuron = State::new();
    let mut time = 0.0;

    let mut times = Vec::new();
    let mut potentials = Vec::new();

    while time < 1_000.0 {
        // record current state
        times.push(time);
        potentials.push(neuron.v);

        // update state
        let syn_i = if time >= 200.0 && time <= 700.0 { 7.0 } else { 0.0 };
        neuron = neuron.step(0.5, syn_i, &config).step(0.5, syn_i, &config);
        time += 1.0;
    }

    let mut fg = Figure::new();
    fg.axes2d().
        set_x_label("time [ms]", &[]).
        set_y_label("potential [mV]", &[]).
        lines(times.iter(), potentials.iter(), &[Caption("Neuron potential over time"), Color("black")]);
    fg.show();
}
