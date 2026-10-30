use kuva::plot::line::LineStyle;
use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::plot::legend::LegendPosition;

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
        let k1 = h * function(*t, x);
        let k2 = h * function(*t + h / 2.0, x + k1 / 2.0);
        let k3 = h * function(*t + h / 2.0, x + k2 / 2.0);
        let k4 = h * function(*t + h, x + k3);
        x += (k1 + 2.0 * k2 +2.0 * k3 + k4) / 6.0;
    }
    
    t_points.into_iter().zip(x_points).collect()
}
fn main() {
    let input_voltage = |t: f64| {
        if (2.0 * t).floor() as i64 % 2 == 0 {
            1.0
        } else {
            -1.0
        }
    };

    let make_voltage_rate = |rc: f64| {
        move |t: f64, voltage_out: f64| {
            (input_voltage(t) - voltage_out) / rc
        }
    };

    let rate_rc_001 = make_voltage_rate(0.01);
    let rate_rc_01 = make_voltage_rate(0.1);
    let rate_rc_1 = make_voltage_rate(1.0);

    let data_rc_001 = rk4(rate_rc_001, 0.0, 0.0, 3.0, 1_000);
    let data_rc_01 = rk4(rate_rc_01, 0.0, 0.0, 3.0, 1_000);
    let data_rc_1 = rk4(rate_rc_1, 0.0, 0.0, 3.0, 1_000);

    let data_input: Vec<(f64, f64)> = (0..=1_000)
        .map(|i| {
            let t = 3.0 * i as f64 / 1_000.0;
            (t, input_voltage(t))
        })
        .collect();

    let plot_input = LinePlot::new()
        .with_data(data_input)
        .with_color("black")
        .with_line_style(LineStyle::Dashed)
        .with_stroke_width(1.5)
        .with_legend("Input voltage");

    let plot_rc_001 = LinePlot::new()
        .with_data(data_rc_001)
        .with_color("steelblue")
        .with_line_style(LineStyle::Solid)
        .with_stroke_width(2.0)
        .with_legend("Output voltage (RC = 0.01)");

    let plot_rc_01 = LinePlot::new()
        .with_data(data_rc_01)
        .with_color("red")
        .with_line_style(LineStyle::Solid)
        .with_stroke_width(2.0)
        .with_legend("Output voltage (RC = 0.1)");

    let plot_rc_1 = LinePlot::new()
        .with_data(data_rc_1)
        .with_color("green")
        .with_line_style(LineStyle::Solid)
        .with_stroke_width(2.0)
        .with_legend("Output voltage (RC = 1)");

    let plots = vec![
        Plot::Line(plot_input),
        Plot::Line(plot_rc_001),
        Plot::Line(plot_rc_01),
        Plot::Line(plot_rc_1),
    ];

    let layout = Layout::auto_from_plots(&plots)
        .with_legend_position(LegendPosition::InsideTopRight)
        .with_x_label("Time")
        .with_y_label("Voltage");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("rc_circuit.svg", svg).unwrap();

  
}

