extern crate izhikevich_neurons;
extern crate gnuplot;

use izhikevich_neurons::{Config, State};
use gnuplot::{Figure, Caption, Color, AxesCommon};

fn main() {
    let config = Config::regular_spiking();
    let mut neuron = State::new();
    let mut time = 0.0;

    let mut times = Vec::new();
    let mut potentials = Vec::new();

    while time < 1_000.0 {
        // record current state
        times.push(time);
        potentials.push(neuron.potential());
        time += 1.0;

        // update state
        let syn_i = if time >= 200.0 && time <= 700.0 { -7.0 } else { 0.0 };
        neuron = neuron.step_1ms(syn_i, &config);
    }

    let mut fg = Figure::new();
    fg.axes2d().
        set_x_label("time [ms]", &[]).
        set_y_label("potential [mV]", &[]).
        lines(times.iter(), potentials.iter(), &[Caption("Neuron potential over time"), Color("black")]);
    fg.show();
}
