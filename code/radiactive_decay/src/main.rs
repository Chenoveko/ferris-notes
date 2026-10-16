use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

fn euler<F>(function: F, initial: f64, a: f64, b: f64, n: usize,) -> Vec<(f64, f64)> 
where
    F: Fn(f64, f64) -> f64
{
    assert!(n > 0, "n must be greater than 0");

    let h: f64 = (b - a) / n as f64;

    let t_points: Vec<f64> = (0..=n)
        .map(|i| a + i as f64 * h)
        .collect();
    
    let mut x_points: Vec<f64> = Vec::with_capacity(n + 1);

    let mut x : f64 = initial;

    for t in t_points.iter() {
        x_points.push(x);
        x += h * function(x, *t)
    }
    
    t_points.into_iter().zip(x_points).collect()
}

fn rk2<F>(function: F, initial: f64, a: f64, b: f64, n: usize,) -> Vec<(f64, f64)> 
where
    F: Fn(f64, f64) -> f64
{
    assert!(n > 0, "n must be greater than 0");

    let h: f64 = (b - a) / n as f64;

    let t_points: Vec<f64> = (0..=n)
        .map(|i| a + i as f64 * h)
        .collect();
    
    let mut x_points: Vec<f64> = Vec::with_capacity(n + 1);

    let mut x : f64 = initial;

    for t in t_points.iter() {
        x_points.push(x);
        let k1: f64 = h * function(x, *t);
        let k2: f64 = h * function(x + k1 / 2.0, *t + h / 2.0);
        x += k2
    }
    
    t_points.into_iter().zip(x_points).collect()
}
fn rk4<F>(function: F, initial: f64, a: f64, b: f64, n: usize,) -> Vec<(f64, f64)> 
where
    F: Fn(f64, f64) -> f64
{
    assert!(n > 0, "n must be greater than 0");

    let h: f64 = (b - a) / n as f64;

    let t_points: Vec<f64> = (0..=n)
        .map(|i| a + i as f64 * h)
        .collect();
    
    let mut x_points: Vec<f64> = Vec::with_capacity(n + 1);

    let mut x : f64 = initial;

    for t in t_points.iter() {
        x_points.push(x);
        let k1: f64 = h * function(x, *t);
        let k2: f64 = h * function(x + k1 / 2.0, *t + h / 2.0);
        let k3: f64 = h * function(x + k2 / 2.0, *t + h / 2.0);
        let k4: f64 = h * function(x + k3, *t + h);
        x += (k1 + 2.0 * k2 +2.0 * k3 + k4) / 6.0;
    }
    
    t_points.into_iter().zip(x_points).collect()
}

fn main() {
    // Carbon-14 (¹⁴C) radioactive decay.
    // Its half-life is approximately 5,730 years.
    let initial: f64 = 100.0;
    let half_life: f64 = 5730.0; // years
    let lambda = 2.0_f64.ln() / half_life; // per year

    let decay = |n: f64, _t: f64| -lambda * n;
    let exact_solution = |t: f64| initial * (-lambda * t).exp();

    let data_euler = euler(decay, initial, 0.0, 30_000.0, 10);
    let data_rk2 = rk2(decay, initial, 0.0, 30_000.0, 10);
    let data_rk4 = rk4(decay, initial, 0.0, 30_000.0, 10);
    let data_exact: Vec<(f64, f64)> = (0..=10000)
        .map(|i| {
            let t = i as f64 * 30_000.0 / 10000.0;
            (t, exact_solution(t))
        })
        .collect();


    let euler_plot = LinePlot::new()
        .with_data(data_euler)
        .with_color("red")
        .with_dotted()
        .with_stroke_width(2.0)
        .with_legend("Euler");

    let rk2_plot = LinePlot::new()
        .with_data(data_rk2)
        .with_color("orange")
        .with_dashed()
        .with_stroke_width(2.0)
        .with_legend("RK2");

    let rk4_plot = LinePlot::new()
        .with_data(data_rk4)
        .with_color("green")
        .with_dashed()
        .with_stroke_width(2.0)
        .with_legend("RK4");

    let exact_plot = LinePlot::new()
        .with_data(data_exact)
        .with_color("blue")
        .with_stroke_width(2.0)
        .with_legend("Exact");

    let plots = vec![
        Plot::Line(euler_plot),
        Plot::Line(rk2_plot),
        Plot::Line(rk4_plot),
        Plot::Line(exact_plot),
    ];

    let layout = Layout::auto_from_plots(&plots)
        .with_title("Radioactive Decay of Carbon-14")
        .with_x_label("$t [years]$")
        .with_y_label("$N(t)$");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("radioactive_decay.svg", svg).unwrap();
}

