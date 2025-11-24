// Xi+1 = r Xi (1-Xi)

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
    let n = 20;

    let seq = logistic_map(r, x0, n);

    for (i, x) in seq.iter().enumerate() {
        println!("x_{} = {:.6}", i, x);
    }
}

