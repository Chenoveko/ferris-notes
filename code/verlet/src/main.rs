use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::{Layout, TickFormat};
use kuva::render::plots::Plot;
use kuva::plot::legend::LegendPosition;

fn verlet<F, G, H>(system: F, potential_energy: G, kinetic_energy: H, initial_state: Vec<f64>, a: f64, b: f64, h: f64) -> (Vec<(f64, f64)>, Vec<(f64, f64)>, Vec<(f64, f64)>, Vec<(f64, f64)>)
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
    G: Fn(f64) -> f64,
    H: Fn(f64) -> f64
{   
    let n = ((b - a) / h).ceil() as usize;
    let t_points: Vec<f64> = (0..n).map(|i| a + i as f64 * h).collect();
    
    let mut x_points: Vec<(f64, f64)> = Vec::new(); // X position points
    let mut y_points: Vec<(f64, f64)> = Vec::new(); // Y position points
    let mut potential_points: Vec<(f64, f64)> = Vec::new(); // Potential energy points
    let mut kinetic_points: Vec<(f64, f64)> = Vec::new(); // Kinetic energy points

    let mut r = initial_state[..2].to_vec(); 
    let mut v = initial_state[2..].to_vec(); 

    let mut v_half: Vec<f64> = v
        .iter()
        .zip(&system(a, &r))
        .map(|(&vi, &ai)| vi + 0.5 * h * ai)
        .collect();

    for t in t_points.iter() {
        x_points.push((*t, r[0]));
        y_points.push((*t, r[1]));

        let radius = (r[0].powi(2) + r[1].powi(2)).sqrt();
        let speed = (v[0].powi(2) + v[1].powi(2)).sqrt();

        potential_points.push((*t, potential_energy(radius)));
        kinetic_points.push((*t, kinetic_energy(speed)));

        r.iter_mut()
            .zip(&v_half)
            .for_each(|(ri, &vi)| *ri += h * vi);

        let k: Vec<f64> = system(*t + h, &r).iter().map(|&ai| h * ai).collect();

        v.iter_mut()
            .zip(v_half.iter().zip(&k))
            .for_each(|(vi, (&vmi, &ki))| *vi = vmi + 0.5 * ki);

        v_half.iter_mut()
            .zip(&k)
            .for_each(|(vmi, &ki)| *vmi += ki);
        }
    
    (x_points, y_points, potential_points, kinetic_points)
}


const G: f64 = 6.67430e-11; // m^3 kg^-1 s^-2
const M_SUN: f64 = 1.9885e30; // kg
const M_EARTH: f64 = 5.9722e24; // kg

fn main() {

    let earth = |_t: f64, position: &[f64]| -> Vec<f64> {
        let x = position[0];
        let y = position[1];
        let r_squared = x * x + y * y;
        let factor = -(G * M_SUN) / r_squared.powf(1.5);

        vec![factor * x, factor * y]
    };

    let potential_energy = |r: f64| -> f64 {
        -G * M_SUN * M_EARTH / r.abs()
    };

    let kinetic_energy = |v: f64| -> f64 {
        0.5 * M_EARTH * v.powi(2)
    };

    let initial_state = vec![1.471e11, 0.0, 0.0, 3.0278e4]; // [x, y, vx, vy]

    let (x_points, y_points, potential_points, kinetic_points) = verlet(earth, potential_energy, kinetic_energy, initial_state, 0.0, 100.0e6, 3600.0);

    let orbit_data: Vec<(f64, f64)> = x_points
        .iter()
        .zip(&y_points)
        .map(|(x, y)| (x.1 / 1.0e11, y.1 / 1.0e11))
        .collect();

    let orbit_plot = LinePlot::new()
        .with_data(orbit_data)
        .with_color("steelblue")
        .with_stroke_width(2.0)
        .with_legend("Earth orbit");

    let plots = vec![Plot::Line(orbit_plot)];

    let layout = Layout::auto_from_plots(&plots)
        .with_legend_position(LegendPosition::InsideTopRight)
        .with_x_label("$x\\[10^{11}\\mathrm{m}]$")
        .with_y_label("$y\\[10^{11}\\mathrm{m}]$")
        .with_tick_format(TickFormat::Auto);


    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("earth_orbit.svg", svg).unwrap();

    let total_points: Vec<(f64, f64)> = potential_points
        .iter()
        .zip(&kinetic_points)
        .map(|(potential, kinetic)| {
            (potential.0, (potential.1 + kinetic.1))
        })
        .collect();

    let potential_plot = LinePlot::new()
        .with_data(
            potential_points
                .iter()
                .map(|&(t, energy)| (t, energy / 1.0e33))
                .collect::<Vec<_>>(),
        )
        .with_color("red")
        .with_stroke_width(2.0)
        .with_legend("Potential energy");

    let kinetic_plot = LinePlot::new()
        .with_data(
            kinetic_points
                .iter()
                .map(|&(t, energy)| (t, energy / 1.0e33))
                .collect::<Vec<_>>(),
        )
        .with_color("blue")
        .with_stroke_width(2.0)
        .with_legend("Kinetic energy");

    let total_plot = LinePlot::new()
        .with_data(
            total_points
                .iter()
                .map(|&(t, energy)| (t, energy / 1.0e33))
                .collect::<Vec<_>>(),
        )
        .with_color("green")
        .with_stroke_width(2.0)
        .with_legend("Total energy");

    let plots = vec![
        Plot::Line(potential_plot),
        Plot::Line(kinetic_plot),
        Plot::Line(total_plot),
    ];

    let layout = Layout::auto_from_plots(&plots)
        .with_x_label("Time [s]")
        .with_y_label("Energy [J]")
        .with_y_label("$Energy\\[10^{33}\\mathrm{J}]$")
        .with_tick_format(TickFormat::Auto);

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("earth_energy.svg", svg).unwrap();

}