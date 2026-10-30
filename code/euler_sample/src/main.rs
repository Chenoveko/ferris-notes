use kuva::plot::line::LineStyle;
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

fn main() {
    let function = |x: f64, t: f64| - x.powi(3) + t.sin();

    let data: Vec<(f64, f64)> = euler(function, 1.32, 0.0, 15.0, 1000);

    let plot = LinePlot::new()
        .with_data(data)
        .with_color("steelblue")
        .with_line_style(LineStyle::Solid)
        .with_stroke_width(2.0);

    let plots = vec![Plot::Line(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_x_label("$t$")
        .with_y_label("$x(t)$");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("euler.svg", svg).unwrap();
}
