extern crate izhikevich_neurons;
extern crate gnuplot;

use izhikevich_neurons::{NeuronConfig, NeuronState, Num};
use gnuplot::{Figure, Caption, Color, AxesCommon};

type NeuronId = u32;
type SynapseId = u32;
type TimeStep = u32;
type Delay = u8;

const MAX_DELAY: u8 = 40;

const STDP_FIRE_RESET: Num = 0.1;
const STDP_DECAY: Num = 0.95;

struct Synapse {
    pre_neuron: NeuronId,
    post_neuron: NeuronId,
    delay: Delay,
    weight: Num,

    // efficiacy derivative used for STDP
    eff_d: Num,

    // ... learning parameters
}

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

struct Network {
    neurons: Vec<Neuron>,
    synapses: Vec<Synapse>,
}

impl Network {
    fn new() -> Network {
        Network {
            neurons: Vec::new(),
            synapses: Vec::new()
        }
    }

    fn create_neuron(&mut self, config: NeuronConfig) -> NeuronId {
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

    fn connect(&mut self, pre_neuron: NeuronId, post_neuron: NeuronId, delay: Delay, weight: Num) -> SynapseId {
        assert!((pre_neuron as usize) < self.neurons.len());
        assert!((post_neuron as usize) < self.neurons.len());
        assert!(delay > 0);
        assert!(delay <= MAX_DELAY);

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


    fn update_state(&mut self, time_step: TimeStep, future_spikes: &mut Vec<Vec<SynapseId>>) {
        //for (i, mut neuron) in network.neurons.iter_mut().enumerate() {
        for i in 0 .. self.neurons.len() {
            let syn_i = self.neurons[i].i_ext + self.neurons[i].i_inp;

            let (new_state, fired) = self.neurons[i].state.step_1ms(syn_i, &self.neurons[i].config);
            self.neurons[i].state = new_state;

            // decay STDP
            self.neurons[i].stdp *= STDP_DECAY;

            if fired {
                // Reset the neurons STDP to a high value.
                self.neurons[i].stdp = STDP_FIRE_RESET;

                println!("Neuron {} fired at {} ms", i, time_step);
                for &syn_id in self.neurons[i].post_synapses.iter() {
                    let future = time_step + self.synapses[syn_id as usize].delay as TimeStep;
                    future_spikes[future as usize % MAX_DELAY as usize].push(syn_id);
                }

                // Excite the synapses that might have led to the firing of the underlying neuron.
                // We do this by adding the synapses pre_neuron's STDP value to the synapses eff_d
                // (efficacy derivative) value.
                //
                // We do not update the synapses weight value immediatly, but only once very while
                // (TODO), so that STDP reflects more LTP (Long Term Potentiation).
                for &syn_id in self.neurons[i].pre_synapses.iter() {
                    let stdp = self.neurons[self.synapses[syn_id as usize].pre_neuron as usize].stdp;
                    self.synapses[syn_id as usize].eff_d += stdp;
                }
            }
        }
    }

    fn update_synapse_weights(&mut self, min_syn_weight: Num, max_syn_weight: Num, eff_d_decay: Num) {
        for syn in self.synapses.iter_mut() {
            let new_weight = syn.weight + syn.eff_d;

            // Restrict synapse weight min_syn_weight .. max_syn_weight
            if new_weight < min_syn_weight {
                syn.weight = min_syn_weight;
            }
            else if new_weight > max_syn_weight {
                syn.weight = max_syn_weight;
            }
            else {
                syn.weight = new_weight;
            }
            syn.eff_d *= eff_d_decay; // decay
        }
    }
}

struct Simulator {
    current_time_step: TimeStep,
    max_delay: usize,

    // We use a cyclic buffer
    // We use (time_step % max_delay) as index into the futures_spike array
    future_spikes: Vec<Vec<SynapseId>>,
}

impl Simulator {
    fn new(max_delay: usize) -> Simulator {
        Simulator {
            current_time_step: 0,
            max_delay: max_delay,
            future_spikes: (0..(max_delay+1)).map(|_| Vec::new()).collect(),
        }
    }

    fn current_time_step(&self) -> TimeStep {
        self.current_time_step
    }
        
    fn step(&mut self, network: &mut Network, external_inputs: &[(NeuronId, TimeStep, Num)]) {
        let time_step = self.current_time_step;

        // Clear all input currents
        for neuron in network.neurons.iter_mut() {
            neuron.i_inp = 0.0;
        }

        // get all synapse input
        {
            let current_spikes = &mut self.future_spikes[(time_step % (self.max_delay as TimeStep)) as usize];
            for &syn_fired in current_spikes.iter() {
                println!("time: {}. input from synapse: {}", time_step, syn_fired); 
                let (weight, pre_neuron, post_neuron) = {
                    let syn = &network.synapses[syn_fired as usize];
                    (syn.weight, syn.pre_neuron, syn.post_neuron)
                };
                network.neurons[post_neuron as usize].i_inp += weight; 

                // whenever a spike arrives here at it's post_neuron, this means, that
                // the pre-neuron fired some time ago (delay time-steps). It can be the 
                // case that the post_neuron has fired ealier, in which case we have to
                // depress the synapse according to the STDP rule.
                network.synapses[syn_fired as usize].eff_d += network.neurons[pre_neuron as usize].stdp - network.neurons[post_neuron as usize].stdp;
            }
            current_spikes.clear();
        }

        // set external inputs
        for &(n_id, at, current) in external_inputs {
            if time_step == at {
                network.neurons[n_id as usize].i_ext = current;
            }
        }

        network.update_state(time_step, &mut self.future_spikes);

        self.current_time_step += 1;
    }
}

fn main() {
    const PARAMS : &'static [(&'static str, &'static str)] = &[
        ("Neuron 1 [7 pA current 200..700ms]", "blue"),
        //("Neuron 2 [2.69 pA current 200..700ms]", 2.69, "red"),
        ("Neuron 2 [0.0 pA current 200..700ms]", "red"),
        //("Neuron 3 [2.7 pA current 200..700ms]", 2.7, "green")
    ];

    let mut network = Network::new();

    let n1 = network.create_neuron(NeuronConfig::regular_spiking());
    let n2 = network.create_neuron(NeuronConfig::regular_spiking());
    //let n3 = network.create_neuron(NeuronConfig::regular_spiking());

    let external_inputs: &[(NeuronId, TimeStep, Num)] = &[
        (n1, 200, 7.0),
        (n1, 701, 0.0)
    ];


    let _ = network.connect(n1, n2, 10, 7.0);
    let _ = network.connect(n1, n2, 5, 7.0);
    let _ = network.connect(n1, n2, 2, 7.0);
    let _ = network.connect(n1, n2, 2, 7.0);
    let _ = network.connect(n2, n2, 20, 7.0);
    let _ = network.connect(n2, n2, 20, 7.0);

    let mut states: Vec<_> = PARAMS.iter().map(|_| Vec::new()).collect();
    let mut sim = Simulator::new(MAX_DELAY as usize);

    while sim.current_time_step() <= 1_000 {
        // record current state
        for (i, neuron) in network.neurons.iter().enumerate() {
            states[i].push(neuron.state);
        }

        sim.step(&mut network, &external_inputs);

        if sim.current_time_step() % 500 == 0 {
            // Update synapse weights every 10 ms
            network.update_synapse_weights(0.0, 10.0, 0.9);
        }
    }

    {
        let mut fg = Figure::new();
        {
            let mut diag = fg.axes2d().
                set_x_label("time (ms)", &[]).
                set_y_label("membrane potential v (mV)", &[]);
            for (i, &p) in PARAMS.iter().enumerate() {
                diag.lines(states[i].iter().enumerate().map(|(i, _)| i as f32), states[i].iter().map(|s| s.potential()), &[Caption(p.0), Color(p.1)]);
            }
        }
        fg.show();
    }


    /*
    let mut fg = Figure::new();
    {
        let mut diag = fg.axes2d().
            set_x_label("membrane potential v (mV)", &[]).
            set_y_label("recovery variable u", &[]);
        for (i, &p) in PARAMS.iter().enumerate() {
            diag.lines(states[i].iter().map(|s| s.potential()), states[i].iter().map(|s| s.recovery()), &[Caption(p.0), Color(p.2)]);
        }
    }
    fg.show();
    */

}
