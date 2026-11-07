use kuva::plot::LinePlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::plot::legend::LegendPosition;

fn leapfrog<F>(system: F, r: Vec<f64>, a: f64, b: f64, h: f64) -> (Vec<(f64, f64)>, Vec<(f64, f64)>)
where
    F: Fn(f64, &[f64]) -> Vec<f64>
{   
    let n = ((b - a) / h).ceil() as usize;
    let t_points: Vec<f64> = (0..n)
        .map(|i| a + i as f64 * h)
        .collect();
    
    let mut x_points: Vec<(f64, f64)> = Vec::new();
    let mut y_points: Vec<(f64, f64)> = Vec::new();

    let mut r1 = r.clone();
    let mut r2: Vec<f64> = r1.iter().zip(system(a, &r1).iter())
        .map(|(value, derivative)| value + h * derivative / 2.0)
        .collect();

    for t in t_points.iter() {
        x_points.push((*t, r1[0]));
        y_points.push((*t, r1[1]));

        r1.iter_mut().zip(system(*t, &r2).iter()).for_each(|(value, derivative)| {
            *value += h * derivative;
        });

        r2.iter_mut().zip(system(*t, &r1).iter()).for_each(|(value, derivative)| {
            *value += h * derivative;
        });
    }
    
    (x_points, y_points)
}

fn main() {
    let system = |_t: f64, state: &[f64]| -> Vec<f64> {
        let x = state[0]; 
        let v = state[1]; 

        vec![
            v,
            v.powi(2) - x - 5.0
        ]
    };

    let (x_data, _v_data) = leapfrog(system, vec![1.0, 0.0], 0.0, 50.0, 0.001);

    let x_plot = LinePlot::new()
        .with_data(x_data)
        .with_color("red")
        .with_stroke_width(2.0);

    //let v_plot = LinePlot::new()
        //.with_data(v_data)
        //.with_color("steelblue")
        //.with_legend("v")
        //.with_stroke_width(2.0);

    let plots = vec![Plot::Line(x_plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_x_label("t")
        .with_y_label("x");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("leapfrog.svg", svg).unwrap();
}
