use plotters::prelude::*;
use std::io

// Computes the logistic map sequence:
// X_{n+1} = r * X_n * (1 - X_n)
fn logistic_map(r: f64, x0: f64, n: usize) -> Vec<f64> {
    let mut x = x0;
    let mut values = Vec::with_capacity(n);

    for _ in 0..n {
        values.push(x);
        x = r * x * (1.0 - x);
    }

    values
}

fn main() {
    let r = 3.9;
    let x0 = 0.5;
    let n = 100;

    // 1. Generate the sequence
    let seq = logistic_map(r, x0, n);

    // 2. Print the sequence
    println!("Logistic map values (r = {r}, x0 = {x0}):");
    for (i, x) in seq.iter().enumerate() { // enumerate() gives (index, value)
        println!("x_{i} = {x}");
    }

    // 3. Plot the sequence to a PNG file

    // Create an 800x600 image file called "logistic.png"
    // BitMapBackend handles file output. Drawing area = canvas.
    let root = BitMapBackend::new("logistic.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap(); /// To paint the entire background white

    // Set up the chart: x from 0..n, y from 0.0..1.0
    let mut chart = ChartBuilder::on(&root)
        .caption("Logistic Map", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0usize..n, 0.0f64..1.0f64)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("step")
        .y_desc("x_n")
        .draw()
        .unwrap();

    // Draw a line where each point is (step, x_step)
    chart
        .draw_series(LineSeries::new(
            seq.iter().enumerate().map(|(i, x)| (i, *x)),
            &BLUE,
        ))
        .unwrap();

    println!("Saved plot to logistic.png");
}
