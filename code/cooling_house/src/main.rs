use kuva::plot::line::LineStyle;
use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::plot::legend::LegendPosition;
use std::f64::consts::PI;

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
        let k1: f64 = h * function(*t, x);
        let k2: f64 = h * function(*t + h / 2.0,x + k1 / 2.0);
        x += k2
    }
    
    t_points.into_iter().zip(x_points).collect()
}

fn main() {
    let ambient_temperature = |t: f64| {
        15.0 + 10.0 * (2.0 * PI * t / 24.0).sin()
    };

    let temperature_rate_variable = |t: f64, temperature: f64| {
        let k = 1.0 / 20.0;
        -k * (temperature - ambient_temperature(t))
    };

    let temperature_rate_constant = |_t: f64, temperature: f64| {
        let k = 1.0 / 20.0;
        -k * (temperature - 15.0)
    };

    let data_var: Vec<(f64, f64)> = rk2(temperature_rate_variable, 30.0, 0.0, 96.0, 1_000);
    let data_cte: Vec<(f64, f64)> = rk2(temperature_rate_constant, 30.0, 0.0, 96.0, 1_000);
    let data_ambient: Vec<(f64, f64)> = (0..=1_000)
        .map(|i| {
            let t = 96.0 * i as f64 / 1_000.0;
            (t, ambient_temperature(t))
        })
        .collect();

    let plot_var = LinePlot::new()
        .with_data(data_var)
        .with_color("steelblue")
        .with_line_style(LineStyle::Solid)
        .with_stroke_width(2.0)
        .with_legend("Indoor temperature (var ambient)");

    let plot_cte = LinePlot::new()
        .with_data(data_cte)
        .with_color("red")
        .with_line_style(LineStyle::Solid)
        .with_stroke_width(2.0)
        .with_legend("Indoor temperature (cte ambient)");

    let plot_ambient = LinePlot::new()
        .with_data(data_ambient)
        .with_color("orange")
        .with_line_style(LineStyle::Dashed)
        .with_stroke_width(2.0)
        .with_legend("Ambient temperature");

    let plots = vec![
        Plot::Line(plot_var),
        Plot::Line(plot_cte),
        Plot::Line(plot_ambient),
    ];

    let layout = Layout::auto_from_plots(&plots)
        .with_legend_position(LegendPosition::InsideTopRight)
        .with_x_label("$Time, t [hours]$")
        .with_y_label("$Indoor temperature T[deg C]$");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("cooling_house.svg", svg).unwrap();
}

