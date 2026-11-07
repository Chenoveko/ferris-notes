use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::plot::legend::LegendPosition;

fn rk45<F>(function: F, initial: f64, a: f64, b: f64, min_step: f64, max_step: f64, tol: f64) -> Vec<(f64, f64)> 
where
    F: Fn(f64, f64) -> f64
{   
    let mut y_points = vec![initial];
    let mut t_points = vec![a];

    let mut t = a;
    let mut h = min_step;

    let mut y = initial;

    while t < b {
        let k1 = function(t, y);
        let k2 = function(t + h / 4.0, y + h * k1 / 4.0);
        let k3 = function(t + 3.0 * h / 8.0, y + h * (3.0 * k1 / 32.0 + 9.0 * k2 / 32.0));
        let k4 = function(t + 12.0 * h / 13.0, y + h * (1932.0 * k1 / 2197.0- 7200.0 * k2 / 2197.0+ 7296.0 * k3 / 2197.0));
        let k5 = function(t + h,y + h * (439.0 * k1 / 216.0- 8.0 * k2+ 3680.0 * k3 / 513.0- 845.0 * k4 / 4104.0));
        let k6 = function(t + h / 2.0,y + h * (-8.0 * k1 / 27.0+ 2.0 * k2- 3544.0 * k3 / 2565.0+ 1859.0 * k4 / 4104.0- 11.0 * k5 / 40.0));

        let error = h * (k1 / 360.0 - 128.0 * k3 / 4275.0 - 2197.0 * k4 / 75240.0 + k5 / 50.0 + 2.0 * k6 / 55.0).abs();

        if error <= tol {
            // Accept the step and update the solution
            y += h * (16.0 * k1 / 135.0 + 6656.0 * k3 / 12825.0 + 28561.0 * k4 / 56430.0 - 9.0 * k5 / 50.0 + 2.0 * k6 / 55.0);
            t += h;
            y_points.push(y);
            t_points.push(t);
        }

        // Adjust the step size, but limit to max_step or min_step
        let delta = 0.84 * (tol / error).powf(0.25);

        if delta <= 0.1 {
            h *= 0.1;
        } else if delta >= 4.0 {
            h *= 4.0;
        } else {
            h *= delta;
        }

        // Clamp the step size to be within the specified bounds
        h = h.clamp(min_step, max_step);
    }

    t_points.into_iter().zip(y_points).collect()
}


fn main() {

    let ode = |_t: f64, y: f64| -> f64 {
        y * (1.0 - y)
    };
    
    let solution = |t: f64| -> f64 {
        0.1 / ((1.0 - 0.1) * (-1.0 * (t - 0.0)).exp() + 0.1)
    };

    let numerical_data = rk45(ode, 0.1, 0.0, 10.0, 0.01, 1.0, 1e-6);
    let analytical_data: Vec<(f64, f64)> = numerical_data.iter().map(|(t, _y)| (*t, solution(*t))).collect();

    let numerical_plot = LinePlot::new()
        .with_data(numerical_data)
        .with_color("red")
        .with_dashed()
        .with_legend("RK45 Numerical Solution")
        .with_stroke_width(2.0);

    let analytical_plot = LinePlot::new()
        .with_data(analytical_data)
        .with_legend("Analytical Solution")
        .with_color("blue")
        .with_stroke_width(2.0);

    let plots = vec![
        Plot::Line(analytical_plot),
        Plot::Line(numerical_plot)
    ];

    let layout = Layout::auto_from_plots(&plots)
        .with_legend_position(LegendPosition::InsideBottomRight)
        .with_x_label("$t$")
        .with_y_label("$y(t)$");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("logistic.svg", svg).unwrap();
}
