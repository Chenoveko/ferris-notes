use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::{Layout, TickFormat};
use kuva::render::plots::Plot;
use kuva::plot::legend::LegendPosition;

fn rk4_step<F>(system: &F, t: f64, state: &[f64], h: f64) -> Vec<f64>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    let k1: Vec<f64> = system(t, state).iter().map(|derivative| h * derivative).collect(); // k1 = h * f(t, r)

    let values_k2: Vec<f64> = state.iter().zip(&k1).map(|(value, k)| value + k / 2.0).collect();
    let k2: Vec<f64> = system(t + h / 2.0, &values_k2).iter().map(|derivative| h * derivative).collect(); // k2 = h * f(t + h/2, r + k1/2)

    let values_k3: Vec<f64> = state.iter().zip(&k2).map(|(value, k)| value + k / 2.0).collect();
    let k3: Vec<f64> = system(t + h / 2.0, &values_k3).iter().map(|derivative| h * derivative).collect(); // k3 = h * f(t + h/2, r + k2/2)

    let values_k4: Vec<f64> = state.iter().zip(&k3).map(|(value, k)| value + k).collect();
    let k4: Vec<f64> = system(t + h, &values_k4).iter().map(|derivative| h * derivative).collect(); // k4 = h * f(t + h, r + k3)

    state.iter().enumerate().map(|(j, value)| {
            value + (k1[j] + 2.0 * k2[j] + 2.0 * k3[j] + k4[j]) / 6.0
        }).collect() // r_next = r + (k1 + 2*k2 + 2*k3 + k4)/6
}

fn rk4_adaptive<F>(system: F, mut r: Vec<f64>, t_max:f64, initial_step:f64, delta:f64) -> (Vec<(f64, f64)>, Vec<(f64, f64)>)
where
    F: Fn(f64, &[f64]) -> Vec<f64>
{
    assert!(t_max > 0.0, "t_max must be greater than 0");
    assert!(initial_step > 0.0, "initial_step must be greater than 0");
    assert!(delta > 0.0, "delta must be greater than 0");

    let mut x_points = vec![(0.0, r[0])];
    let mut y_points = vec![(0.0, r[1])];

    let mut t = 0.0;
    let mut h = initial_step;

    while t < t_max {
        // Make a big step 2h
        let r1 = rk4_step(&system, t, &r, 2.0 * h);
        
        // Make two small steps h
        let midpoint = rk4_step(&system, t, &r, h);
        let r2 = rk4_step(&system, t + h, &midpoint, h);

        // Compute the value of the precision ratio
        let dx = r1[0] - r2[0]; // Error in x
        let dy = r1[1] - r2[1]; // Error in y
        let rho = 30.0 * h * delta / (dx * dx + dy * dy).sqrt(); // Precision ratio

        // Adjust the step size based on the precision ratio
        if rho >= 1.0 {
            // Accept the step and update the state
            t += 2.0 *h;
            h *= rho.powf(0.25).min(2.0); // Increase the step size, but limit it to a maximum of double the current step size
            r = r2;
            x_points.push((t, r[0]));
            y_points.push((t, r[1]));
        } else {
            // Reject the step and reduce the step size
            h *= rho.powf(0.25); // Decrease the step size, but limit it to a minimum of 10% of the current step size
        }
    }

   
    
    (x_points, y_points)
}

const G: f64 = 6.67430e-11; // m^3 kg^-1 s^-2
const M_SUN: f64 = 1.9885e30; // kg
const SCALE: f64 = 1.0e12;

fn main() {
    
    let comet = |_t: f64, state: &[f64]| -> Vec<f64> {
        let x = state[0]; 
        let y = state[1]; 
        let vx = state[2]; 
        let vy = state[3];

        vec![
            vx,
            vy,
            -(G * M_SUN) * x / (x.powi(2) + y.powi(2)).powf(1.5),
            -(G * M_SUN) * y / (x.powi(2) + y.powi(2)).powf(1.5)
        ]
    };

    let initial = vec![4.0e12, 0.0, 0.0, 500.0]; // [x, y, v_x, v_y]

    let t_max = 3.2e9; // Approximately 2 orbits of Comet
    let initial_step = 1.0e4; // Initial step size of 10_000 Km
    let delta = 1.0e3/365.25/3600.0; // Desired precision for adaptive step size in Km/s

    let (x_data, y_data) = rk4_adaptive(comet, initial, t_max, initial_step, delta);

    let orbit_data: Vec<(f64, f64)> = x_data
        .iter()
        .zip(&y_data)
        .map(|((_, x), (_, y))| (x / SCALE, y / SCALE))
        .collect();

    let orbit_plot = LinePlot::new()
        .with_data(orbit_data)
        .with_color("steelblue")
        .with_stroke_width(2.0)
        .with_legend("Comet orbit");

    let plots = vec![Plot::Line(orbit_plot)];

    let layout = Layout::auto_from_plots(&plots)
        .with_legend_position(LegendPosition::InsideTopRight)
        .with_x_label("$x\\[10^{12}\\mathrm{m}]$")
        .with_y_label("$y\\[10^{12}\\mathrm{m}]$")
        .with_tick_format(TickFormat::Auto);


    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("comet_orbit.svg", svg).unwrap();
}
