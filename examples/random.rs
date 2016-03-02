extern crate izhikevich_neurons;
extern crate gnuplot;
extern crate rand;
extern crate closed01;

use izhikevich_neurons::{NeuronConfig, Simulator, Network, FireRecorder};
use izhikevich_neurons::event_queue::{EventQueue, Event};
use gnuplot::{Figure, /* Caption, */ Color, AxesCommon, PlotOption, AutoOption};
use rand::Rng;
use closed01::Closed01;

const INPUTS: usize = 9;

fn main() {
    let mut rng = rand::thread_rng();
    let mut fire_recorder = FireRecorder::new();
    let mut network = Network::new();

    let input_neurons = network.n_neurons_of(INPUTS, &mut |_| NeuronConfig::chattering());
    let hidden_neurons = network.n_neurons_of(18,
                                              &mut |_| {
                                                  NeuronConfig::excitatory(Closed01::new(rng.gen()))
                                              });
    let _output_neurons = network.n_neurons_of(1, &mut |_| NeuronConfig::regular_spiking());

    for _ in 1..10 {
        network.connect_all_with(&input_neurons,
                                 &hidden_neurons,
                                 &mut |_, _| Some((rng.gen_range(2, 56), rng.gen_range(0.0, 5.0))));
        // network.connect_all_with(&hidden_neurons,
        // &input_neurons,
        // &mut |_, _| Some((rng.gen_range(2, 56), rng.gen_range(0.0, 5.0))));
        //
    }
    let mut sim = Simulator::new(network.max_delay() as usize);

    let mut external_inputs = EventQueue::new();

    for _ in 1..2000 {
        let event = Event {
            at: rng.gen_range(1, 10_000),
            neuron: *rng.choose(&input_neurons).unwrap(),
            weight: rng.gen_range(0.0, 4.0),
        };
        let event_down = Event {
            at: event.at + rng.gen_range(2, 10),
            neuron: event.neuron,
            weight: -event.weight,
        };

        external_inputs.push(event);
        external_inputs.push(event_down);
    }

    while sim.current_time_step() <= 10_000 {

        while let Some(ev) = external_inputs.pop_next_event_at(sim.current_time_step()) {
            let input = network.get_external_input(ev.neuron);
            network.set_external_input(ev.neuron, input + ev.weight);
        }

        sim.step(&mut network,
                 &mut |neuron_id, timestep| {
                     fire_recorder.record(neuron_id, timestep);
                 });

        if sim.current_time_step() % 500 == 0 {
            // Update synapse weights every 500 ms
            network.update_synapse_weights(0.0, 10.0, 0.9);
        }
    }

    {
        let mut fg = Figure::new();
        {
            let mut diag = fg.axes2d()
                             .set_y_ticks(Some((AutoOption::Fix(1.0), 0)), &[], &[])
                             .set_y_range(AutoOption::Fix(0.0),
                                          AutoOption::Fix((network.total_neurons() - 1) as f64))
                             .set_x_label("time (ms)", &[])
                             .set_y_label("neuron id", &[]);

            diag.points(fire_recorder.events.iter().map(|&(_, t)| t),
                        fire_recorder.events.iter().map(|&(i, _)| i.index()),
                        &[PlotOption::PointSymbol('S'),
                          Color("black"),
                          PlotOption::PointSize(0.2)]);
        }
        fg.show();
    }
}
