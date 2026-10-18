use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

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
    let function = |x: f64, u: f64|  1.0 / ((1.0 - u).powi(2) * x.powi(2) + u.powi(2));

    let data_u = rk4(function, 1.0, 0.0, 0.99, 1000);

    let data_t: Vec<(f64, f64)> = data_u
        .into_iter()
        .map(|(u, x)| (u / (1.0 - u), x))
        .collect();

    let plot = LinePlot::new()
        .with_data(data_t)
        .with_color("red")
        .with_stroke_width(2.0);

    let plots = vec![Plot::Line(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("RK4 Approximation of x(t) on an Infinite Domain")
        .with_x_label("$t$")
        .with_y_label("$x(t)$");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("rk4_infinite.svg", svg).unwrap();
}