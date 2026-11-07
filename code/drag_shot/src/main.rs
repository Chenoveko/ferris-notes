use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use ndarray::{Array1, ArrayView1};
use std::time::Instant;

fn rk4_step<F>(function: &F, t: f64, x: &Array1<f64>, h: f64) -> Array1<f64>
where
    F: Fn(f64, &Array1<f64>) -> Array1<f64>
{
    let k1 = function(t, x);

    let x2 = x + &k1.mapv(|v| v * (h / 2.0));
    let k2 = function(t + h / 2.0, &x2);

    let x3 = x + &k2.mapv(|v| v * (h / 2.0));
    let k3 = function(t + h / 2.0, &x3);

    let x4 = x + &k3.mapv(|v| v * h);
    let k4 = function(t + h, &x4);

    let sum = &k1 + &k2.mapv(|v| 2.0 * v) + &k3.mapv(|v| 2.0 * v) + &k4;

    x + &sum.mapv(|v| v * (h / 6.0))
}

fn rk4<F>(function: F, initial: Array1<f64>, t_0: f64, h: f64) -> Vec<(f64, Array1<f64>)>
where
    F: Fn(f64, &Array1<f64>) -> Array1<f64>
{
    let mut points = Vec::new();
    let mut t = t_0;
    let mut x = initial;

    points.push((t, x.clone()));

    while x[2] >= 0.0 {
        x = rk4_step(&function, t, &x, h);
        t += h;
        points.push((t, x.clone()));
    }

    points
}

fn ab4<F>(function: F, initial: Array1<f64>, t_0: f64, h: f64) -> Vec<(f64, Array1<f64>)>
where
    F: Fn(f64, &Array1<f64>) -> Array1<f64>
{
    let mut points = Vec::new();
    let mut t = t_0;
    let mut x = initial;

    points.push((t, x.clone()));

    // Begin with RK4 to get the first 4 points
    for _ in 0..3 {
        x = rk4_step(&function, t, &x, h);
        t += h;
        points.push((t, x.clone()));
    }

    while x[2] >= 0.0 {
        let f_n = function(t, &x);
        let f_n1 = function(t - h, &points[points.len() -2].1);
        let f_n2 = function(t - 2.0 * h, &points[points.len() -3].1);
        let f_n3 = function(t - 3.0 * h, &points[points.len() -4].1);
        x = &x + &(&f_n.mapv(|v| 55.0 * v) - &f_n1.mapv(|v| 59.0 * v) + &f_n2.mapv(|v| 37.0 * v) - &f_n3.mapv(|v| 9.0 * v)).mapv(|v| v * (h / 24.0));
        t += h;
        points.push((t, x.clone()));
    }

    points
}

const Rho_0: f64 = 1.225; // kg/m^3
const H: f64 = 8500.0; // m
const A: f64 = 0.0189; // m^2
const Cd: f64 = 0.24; // dimensionless
const M: f64 = 43.2; // kg
const G: f64 = 9.80; // m/s^2

fn main() {
    let air_density = |z: f64| Rho_0 * (-z / H).exp();

    let acceleration = |v: ArrayView1<'_, f64>, z: f64| {
        let a_g = Array1::from_vec(vec![0.0, 0.0, -G]);
        let rho = air_density(z);
        let v_norm = v.dot(&v).sqrt();
        let a_d = &v * (-0.5 * rho * A * Cd * v_norm / M);

        a_g + a_d
    };

    // ODE: state = [x, y, z, vx, vy, vz], where (x, y, z) is position and (vx, vy, vz) is velocity
    let ode = |_: f64, state: &Array1<f64>| {
        let position = state.slice(ndarray::s![0..3]);
        let velocity = state.slice(ndarray::s![3..6]);

        Array1::from_vec(vec![
            velocity[0],
            velocity[1],
            velocity[2],
            acceleration(velocity, position[2])[0],
            acceleration(velocity, position[2])[1],
            acceleration(velocity, position[2])[2],
        ])
    };

    // Initial conditions
    let v_0: f64 = 650.0;
    let angle = 38.0_f64.to_radians();

    let initial = Array1::from_vec(vec![
        0.0,                      // x
        0.0,                      // y
        0.0,                      // z
        v_0 * angle.cos(),        // vx
        0.0,                      // vy
        v_0 * angle.sin(),        // vz
    ]);
    
    // Run RK4
    let start_rk4 = Instant::now();
    let points_rk4 = rk4(ode, initial.clone(), 0.0, 0.01);
    let duration_rk4 = start_rk4.elapsed();

    let (t_impact, impact_state) = points_rk4.last().unwrap();

    let max_height = points_rk4
        .iter()
        .map(|(_, state)| state[2])
        .fold(f64::NEG_INFINITY, f64::max);

    let impact_speed = impact_state
        .slice(ndarray::s![3..6])
        .dot(&impact_state.slice(ndarray::s![3..6]))
        .sqrt();

    println!("RK4: ToF = {:.2} s, Range = {:.2} m, Max Height = {:.2} m, Impact Velocity = {:.2} m/s", t_impact, impact_state[0], max_height, impact_speed);

    // Run AB4
    let start_ab4 = Instant::now();
    let points_ab4 = ab4(ode, initial.clone(), 0.0, 0.01);
    let duration_ab4 = start_ab4.elapsed();

    let (t_impact, impact_state) = points_ab4.last().unwrap();

    let max_height = points_ab4
        .iter()
        .map(|(_, state)| state[2])
        .fold(f64::NEG_INFINITY, f64::max);

    let impact_speed = impact_state
        .slice(ndarray::s![3..6])
        .dot(&impact_state.slice(ndarray::s![3..6]))
        .sqrt();

    println!("AB4: ToF = {:.2} s, Range = {:.2} m, Max Height = {:.2} m, Impact Velocity = {:.2} m/s", t_impact, impact_state[0], max_height, impact_speed);
    println!("Computation time: RK4 {} ms, AB4 {} ms",duration_rk4.as_millis(), duration_ab4.as_millis());
    println!("AB4 vs RK4: {}",duration_rk4.as_millis() as f64 / duration_ab4.as_millis() as f64);

    // Plot trajetory
    let data: Vec<(f64, f64)> = points_rk4
        .iter()
        .map(|(_, state)| (state[0], state[2])) // (posición x, altura z)
        .collect();

    let plot = LinePlot::new()
        .with_data(data)
        .with_color("red")
        .with_stroke_width(2.0);

    let plots = vec![Plot::Line(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_x_label("Position x [m]")
        .with_y_label("Height z [m]")
        .with_y_axis_min(0.0);


    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("drag_shot.svg", svg).unwrap();
}
