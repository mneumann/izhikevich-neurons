extern crate izhikevich_neurons;
extern crate gnuplot;

use izhikevich_neurons::{NeuronConfig, NeuronId, TimeStep, Num, Simulator, Network};
use gnuplot::{Figure, Caption, Color, AxesCommon, PlotOption};

#[derive(Debug)]
struct FireRecorder {
    events: Vec<(NeuronId, TimeStep)>
}

impl FireRecorder {
    pub fn new() -> FireRecorder {
        FireRecorder {
            events: Vec::new(),
        }
    }

    pub fn record(&mut self, neuron_id: NeuronId, time_step: TimeStep) {
        self.events.push((neuron_id, time_step));
    }
}

fn main() {
    const PARAMS : &'static [(&'static str, &'static str)] = &[
        ("Neuron 1 [7 pA current 200..700ms]", "blue"),
        //("Neuron 2 [2.69 pA current 200..700ms]", 2.69, "red"),
        ("Neuron 2 [0.0 pA current 200..700ms]", "red"),
        //("Neuron 3 [2.7 pA current 200..700ms]", 2.7, "green")
    ];

    let mut fire_recorder = FireRecorder::new();
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
    let mut sim = Simulator::new(network.max_delay() as usize);

    while sim.current_time_step() <= 1_000 {
        // record current state
        for (i, &neuron_state) in network.save_state().iter().enumerate() {
            states[i].push(neuron_state);
        }

        sim.step(&mut network, &external_inputs, |neuron_id, timestep| {
            fire_recorder.record(neuron_id, timestep);
        });

        if sim.current_time_step() % 500 == 0 {
            // Update synapse weights every 10 ms
            network.update_synapse_weights(0.0, 10.0, 0.9);
        }
    }

    {
        println!("{:?}", fire_recorder);
        let mut fg = Figure::new();
        {
            let mut diag = fg.axes2d().
                set_x_label("time (ms)", &[]).
                set_y_label("neuron id", &[]);

            diag.points(
                fire_recorder.events.iter().map(|&(_, t)| t),
                fire_recorder.events.iter().map(|&(i, _)| i),
                &[PlotOption::PointSymbol('S'), Color("black"), PlotOption::PointSize(1.25)]);
        }
        fg.show();
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
