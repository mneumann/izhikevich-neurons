extern crate gnuplot;
extern crate izhikevich_neurons;

use izhikevich_neurons::{FireRecorder, Network, NeuronConfig, NeuronId, Num, Simulator, Timestep};
use gnuplot::{AutoOption, AxesCommon, /*Caption,*/ Color, Figure, PlotOption};

fn main() {
    // const PARAMS : &'static [(&'static str, &'static str)] = &[
    // ("Neuron 1 [7 pA current 200..700ms]", "blue"),
    // ("Neuron 2 [2.69 pA current 200..700ms]", 2.69, "red"),
    // ("Neuron 2 [0.0 pA current 200..700ms]", "red"),
    // ("Neuron 3 [2.7 pA current 200..700ms]", 2.7, "green")
    // ];
    //

    let mut fire_recorder = FireRecorder::new();
    let mut network = Network::new();

    let input_neurons = network.n_neurons_of(9, &mut |_| NeuronConfig::regular_spiking());
    let middle_neurons = network.n_neurons_of(18, &mut |_| NeuronConfig::regular_spiking());
    let output_neurons = network.n_neurons_of(1, &mut |_| NeuronConfig::regular_spiking());

    // let n1 = network.create_neuron(NeuronConfig::regular_spiking());
    // let n2 = network.create_neuron(NeuronConfig::regular_spiking());
    // let n3 = network.create_neuron(NeuronConfig::regular_spiking());

    let pattern1: [u8; 9] = [0, 0, 0, 1, 1, 1, 0, 0, 0];

    let pattern2: [u8; 9] = [0, 0, 1, 0, 1, 0, 1, 0, 0];

    let mut external_inputs: Vec<(NeuronId, Timestep, Num)> = Vec::new();

    for (i, &v) in pattern1.iter().enumerate() {
        external_inputs.push((input_neurons[i], 0, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 2000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 4000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 6000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 7000, if v == 0 { 0.0 } else { 4.0 }));
    }

    for (i, &v) in pattern2.iter().enumerate() {
        external_inputs.push((input_neurons[i], 1000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 3000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 5000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 6500, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 7500, if v == 0 { 0.0 } else { 4.0 }));
    }

    // let external_inputs: &[(NeuronId, TimeStep, Num)] = &[
    // (n1, 200, 7.0),
    // (n1, 701, 0.0)
    // ];
    //

    // connect every input neuron with every output neuron.
    network.connect_all(&input_neurons[..], &middle_neurons[..], 2, 2.0);
    network.connect_all(&input_neurons[..], &middle_neurons[..], 3, 2.0);
    network.connect_all(&input_neurons[..], &middle_neurons[..], 5, 2.0);

    network.connect_all(&middle_neurons[..], &output_neurons[..], 1, 1.0);

    // for i in 0..4 {
    // for j in 0..4 {
    // if i != j {
    // network.connect(output_neurons[i], output_neurons[j], 2, -70.0);
    // }
    // }
    // }
    //

    // let _ = network.connect(n1, n2, 10, 7.0);
    // let _ = network.connect(n1, n2, 5, 7.0);
    // let _ = network.connect(n1, n2, 2, 7.0);
    // let _ = network.connect(n1, n2, 2, 7.0);
    // let _ = network.connect(n2, n2, 20, 7.0);
    // let _ = network.connect(n2, n2, 20, 7.0);
    //

    // let mut states: Vec<_> = PARAMS.iter().map(|_| Vec::new()).collect();
    let mut sim = Simulator::new(network.max_delay() as usize);

    while sim.current_time_step() <= 10_000 {
        // record current state
        // for (i, &neuron_state) in network.save_state().iter().enumerate() {
        // states[i].push(neuron_state);
        // }

        // set external inputs

        for &(neuron_id, at, current) in external_inputs.iter() {
            if sim.current_time_step() == at {
                network.set_external_input(neuron_id, current);
            }
        }

        sim.step(&mut network, &mut |neuron_id, timestep| {
            fire_recorder.record(neuron_id, timestep);
        });

        if sim.current_time_step() % 500 == 0 {
            // Update synapse weights every 10 ms
            if sim.current_time_step() < 6001 {
                network.update_synapse_weights(0.0, 10.0, 0.9);
            }
        }
    }

    {
        // println!("{:?}", fire_recorder);
        let mut fg = Figure::new();
        {
            let mut diag = fg.axes2d()
                .set_y_ticks(Some((AutoOption::Fix(1.0), 0)), &[], &[])
                .set_y_range(
                    AutoOption::Fix(0.0),
                    AutoOption::Fix((network.total_neurons() - 1) as f64),
                )
                .set_x_label("time (ms)", &[])
                .set_y_label("neuron id", &[]);

            diag.points(
                fire_recorder.events.iter().map(|&(_, t)| t),
                fire_recorder.events.iter().map(|&(i, _)| i.index()),
                &[
                    PlotOption::PointSymbol('S'),
                    Color("black"),
                    PlotOption::PointSize(0.2),
                ],
            );
        }
        fg.show();
    }

    // {
    // let mut fg = Figure::new();
    // {
    // let mut diag = fg.axes2d().
    // set_x_label("time (ms)", &[]).
    // set_y_label("membrane potential v (mV)", &[]);
    // for (i, &p) in PARAMS.iter().enumerate() {
    // diag.lines(states[i].iter().enumerate().map(|(i, _)| i as f32), states[i].iter().map(|s| s.potential()), &[Caption(p.0), Color(p.1)]);
    // }
    // }
    // fg.show();
    // }
    //

    // let mut fg = Figure::new();
    // {
    // let mut diag = fg.axes2d().
    // set_x_label("membrane potential v (mV)", &[]).
    // set_y_label("recovery variable u", &[]);
    // for (i, &p) in PARAMS.iter().enumerate() {
    // diag.lines(states[i].iter().map(|s| s.potential()), states[i].iter().map(|s| s.recovery()), &[Caption(p.0), Color(p.2)]);
    // }
    // }
    // fg.show();
    //
}
