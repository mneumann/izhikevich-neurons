extern crate closed01;

use closed01::Closed01;

/// We use this numerical type for all calculations.
pub type Num = f32;

/// Represents the state of a neuron.
#[derive(Copy, Clone)]
pub struct NeuronState {
    /// membrane potential of neuron (in mV)
    v: Num,

    /// recovery variable
    u: Num,
}

/// At which potential the neuron's potential is reset to `c`.
const RESET_THRESHOLD: Num = 30.0;

impl NeuronState {
    pub fn new() -> NeuronState {
        NeuronState {
            v: -70.0,
            u: -14.0,
        }
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
}

/// The Neuron's configuration parameters.
pub struct NeuronConfig {
    /// Rate of recovery.
    a: Num,

    /// Sensitivity of recovery variable `u` to the membrane potential `v`.
    b: Num,

    /// After-spike reset value of membrane potential `v`.
    c: Num,

    /// After-spike reset of recovery variable `u`.
    d: Num,
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

#[inline(always)]
fn dv(u: Num, v: Num, i_syn: Num) -> Num {
    (0.04 * v + 5.0) * v + 140.0 - u + i_syn
}

#[inline(always)]
fn du(u: Num, v: Num, a: Num, b: Num) -> Num {
    a * (b * v - u)
}

impl NeuronState {
    #[inline(always)]
    fn calc(self, dt: Num, i_syn: Num, config: &NeuronConfig) -> NeuronState {
        NeuronState {
            v: self.v + dt * dv(self.u, self.v, i_syn),
            u: self.u + dt * du(self.u, self.v, config.a, config.b),
        }
    }

    /// Calculate the new state after 1 ms.
    #[inline(always)]
    pub fn step_1ms(self, i_syn: Num, config: &NeuronConfig) -> (NeuronState, bool) {
        if self.v < RESET_THRESHOLD {
            (self.calc(0.5, i_syn, config).calc(0.5, i_syn, config),
             false)
        } else {
            (NeuronState {
                v: config.c,
                u: self.u + config.d,
            },
             true)
        }
    }
}

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

pub type NeuronId = u32;
pub type SynapseId = u32;
pub type TimeStep = u32;
pub type Delay = u8;

const MAX_DELAY: u8 = 64;
const STDP_FIRE_RESET: Num = 0.1;
const STDP_DECAY: Num = 0.95;

struct Neuron {
    state: NeuronState,
    config: NeuronConfig,
    i_ext: Num,
    i_inp: Num,

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
    stdp: Num,

    // XXX: Rename to Connection and incoming/outgoing?
    pre_synapses: Vec<SynapseId>,
    post_synapses: Vec<SynapseId>,
}

struct Synapse {
    pre_neuron: NeuronId,
    post_neuron: NeuronId,
    delay: Delay,
    weight: Num,

    // efficiacy derivative used for STDP
    eff_d: Num, // ... learning parameters
}

pub struct Network {
    neurons: Vec<Neuron>,
    synapses: Vec<Synapse>,
    max_delay: Delay,
}

impl Network {
    pub fn new() -> Network {
        Network {
            neurons: Vec::new(),
            synapses: Vec::new(),
            max_delay: 0,
        }
    }

    pub fn n_neurons_of<F>(&mut self, n: usize, f: F) -> Vec<NeuronId>
        where F: Fn(usize) -> NeuronConfig
    {
        (0..n).map(|i| self.create_neuron(f(i))).collect()
    }

    pub fn save_state(&self) -> Vec<NeuronState> {
        self.neurons.iter().enumerate().map(|(_, n)| n.state).collect()
    }

    pub fn create_neuron(&mut self, config: NeuronConfig) -> NeuronId {
        let neuron = Neuron {
            state: NeuronState::new(),
            config: config,
            stdp: 0.0,
            i_ext: 0.0,
            i_inp: 0.0,
            pre_synapses: Vec::new(),
            post_synapses: Vec::new(),
        };
        let neuron_id = self.neurons.len() as u32;
        self.neurons.push(neuron);
        return neuron_id;
    }

    pub fn max_delay(&self) -> Delay {
        self.max_delay
    }

    pub fn total_neurons(&self) -> usize {
        self.neurons.len()
    }

    pub fn connect_all(&mut self, a: &[NeuronId], b: &[NeuronId], delay: Delay, weight: Num) {
        for &i in a.iter() {
            for &o in b.iter() {
                let _ = self.connect(i, o, delay, weight);
            }
        }
    }

    pub fn connect(&mut self,
                   pre_neuron: NeuronId,
                   post_neuron: NeuronId,
                   delay: Delay,
                   weight: Num)
                   -> SynapseId {
        assert!((pre_neuron as usize) < self.neurons.len());
        assert!((post_neuron as usize) < self.neurons.len());
        assert!(delay > 0);
        assert!(delay <= MAX_DELAY);

        if delay > self.max_delay {
            self.max_delay = delay;
        }

        let synapse = Synapse {
            pre_neuron: pre_neuron,
            post_neuron: post_neuron,
            delay: delay,
            weight: weight,
            eff_d: 0.0,
        };
        let synapse_id = self.synapses.len() as u32;

        self.synapses.push(synapse);
        self.neurons[pre_neuron as usize].post_synapses.push(synapse_id);
        self.neurons[post_neuron as usize].pre_synapses.push(synapse_id);

        return synapse_id;
    }

    fn update_state<F>(&mut self,
                       time_step: TimeStep,
                       future_spikes: &mut Vec<Vec<SynapseId>>,
                       mut fired_callback: F)
        where F: FnMut(NeuronId, TimeStep)
    {
        // for (i, mut neuron) in network.neurons.iter_mut().enumerate() {
        for i in 0..self.neurons.len() {
            let syn_i = self.neurons[i].i_ext + self.neurons[i].i_inp;

            let (new_state, fired) = self.neurons[i].state.step_1ms(syn_i, &self.neurons[i].config);
            self.neurons[i].state = new_state;

            // decay STDP
            self.neurons[i].stdp *= STDP_DECAY;

            if fired {
                fired_callback(i as NeuronId, time_step);

                // Reset the neurons STDP to a high value.
                self.neurons[i].stdp = STDP_FIRE_RESET;

                for &syn_id in self.neurons[i].post_synapses.iter() {
                    let future = time_step + self.synapses[syn_id as usize].delay as TimeStep;
                    let max_delay = future_spikes.len();
                    future_spikes[future as usize % max_delay].push(syn_id);
                }

                // Excite the synapses that might have led to the firing of the underlying neuron.
                // We do this by adding the synapses pre_neuron's STDP value to the synapses eff_d
                // (efficacy derivative) value.
                //
                // We do not update the synapses weight value immediatly, but only once very while
                // (TODO), so that STDP reflects more LTP (Long Term Potentiation).
                for &syn_id in self.neurons[i].pre_synapses.iter() {
                    let stdp = self.neurons[self.synapses[syn_id as usize].pre_neuron as usize]
                                   .stdp;
                    self.synapses[syn_id as usize].eff_d += stdp;
                }
            }
        }
    }

    pub fn update_synapse_weights(&mut self,
                                  min_syn_weight: Num,
                                  max_syn_weight: Num,
                                  eff_d_decay: Num) {
        for syn in self.synapses.iter_mut() {
            let new_weight = syn.weight + syn.eff_d;

            // Restrict synapse weight min_syn_weight .. max_syn_weight
            if new_weight < min_syn_weight {
                syn.weight = min_syn_weight;
            } else if new_weight > max_syn_weight {
                syn.weight = max_syn_weight;
            } else {
                syn.weight = new_weight;
            }
            syn.eff_d *= eff_d_decay; // decay
        }
    }
}

pub struct Simulator {
    current_time_step: TimeStep,
    max_delay: usize,

    // We use a cyclic buffer
    // We use (time_step % max_delay) as index into the futures_spike array
    future_spikes: Vec<Vec<SynapseId>>,
}

impl Simulator {
    pub fn new(max_delay: usize) -> Simulator {
        Simulator {
            current_time_step: 0,
            max_delay: max_delay,
            future_spikes: (0..(max_delay + 1)).map(|_| Vec::new()).collect(),
        }
    }

    pub fn current_time_step(&self) -> TimeStep {
        self.current_time_step
    }

    pub fn step<F>(&mut self,
                   network: &mut Network,
                   external_inputs: &[(NeuronId, TimeStep, Num)],
                   fired_callback: F)
        where F: FnMut(NeuronId, TimeStep)
    {
        let time_step = self.current_time_step;

        // Clear all input currents
        for neuron in network.neurons.iter_mut() {
            neuron.i_inp = 0.0;
        }

        // get all synapse input
        {
            let current_spikes =
                &mut self.future_spikes[(time_step % (self.max_delay as TimeStep)) as usize];
            for &syn_fired in current_spikes.iter() {
                // println!("time: {}. input from synapse: {}", time_step, syn_fired);
                let (weight, pre_neuron, post_neuron) = {
                    let syn = &network.synapses[syn_fired as usize];
                    (syn.weight, syn.pre_neuron, syn.post_neuron)
                };
                network.neurons[post_neuron as usize].i_inp += weight;

                // whenever a spike arrives here at it's post_neuron, this means, that
                // the pre-neuron fired some time ago (delay time-steps). It can be the
                // case that the post_neuron has fired ealier, in which case we have to
                // depress the synapse according to the STDP rule.
                network.synapses[syn_fired as usize].eff_d += network.neurons[pre_neuron as usize]
                                                                  .stdp -
                                                              network.neurons[post_neuron as usize]
                                                                  .stdp;
            }
            current_spikes.clear();
        }

        // set external inputs
        for &(n_id, at, current) in external_inputs {
            if time_step == at {
                network.neurons[n_id as usize].i_ext = current;
            }
        }

        network.update_state(time_step, &mut self.future_spikes, fired_callback);

        self.current_time_step += 1;
    }
}

#[derive(Debug)]
pub struct FireRecorder {
    pub events: Vec<(NeuronId, TimeStep)>,
}

impl FireRecorder {
    pub fn new() -> FireRecorder {
        FireRecorder { events: Vec::new() }
    }

    pub fn record(&mut self, neuron_id: NeuronId, time_step: TimeStep) {
        self.events.push((neuron_id, time_step));
    }
}

// impl PotentialRecorder {
// }
