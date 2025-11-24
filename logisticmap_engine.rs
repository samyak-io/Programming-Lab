// The logistic map describes a sequence defined by:
//    X_{n+1} = r * X_n * (1 - X_n)
// It models population growth with limited resources and can show chaotic behavior.

// We wrap the state of the logistic map in a struct.
// In Rust, a struct is a custom data type that groups related values.
struct LogisticMap {
    r: f64, // The parameter 'r' controlling growth/chaos. f64 = 64-bit floating point.
    x: f64, // The current value X_n in the sequence.
}

impl LogisticMap {
    // Constructor: creates a new logistic map with parameter r and initial value x0.
    // `Self` refers to LogisticMap itself.
    fn new(r: f64, x0: f64) -> Self {
        Self { r, x: x0 } // Field init shorthand: r → r, x → x0.
    }
}

// Implementing the Iterator trait allows the logistic map to produce a sequence
// lazily — one value at a time — just like a mathematical recurrence relation.
impl Iterator for LogisticMap {
    type Item = f64; // Each iteration returns an f64 value (X_n).

    // The core of the logistic map:
    // Every call to next() returns the current value X_n,
    // then updates x → X_{n+1} using the formula r * x * (1 - x).
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.x; // Save X_n. `let` creates a local variable.

        // Update internal state:
        //   X_{n+1} = r * X_n * (1 - X_n)
        // This overwrites self.x so the next iteration starts from the new value.
        self.x = self.r * self.x * (1.0 - self.x);

        // Return X_n wrapped in Some(), because Iterator::next() uses Option.
        Some(current)
    }
}

fn main() {
    // Choose r in the chaotic regime so the sequence becomes unpredictable.
    let r = 3.9;

    // Initial value X_0 between 0 and 1.
    let x0 = 0.5;

    // Create the iterator (like defining the recurrence relation).
    let mut iter = LogisticMap::new(r, x0);

    // Warm-up: initial values often lie on a “transient” path.
    // This loop runs next() 1000 times and ignores the results.
    for _ in 0..1000 {
        iter.next();
    }

    // Now take the next 20 values from the iterator.
    // `.take(20)` limits the iterator to 20 iterations.
    // `.enumerate()` attaches an index i.
    for (i, x) in iter.take(20).enumerate() {
        // `println!` is a macro for formatted printing.
        // {:.6} formats x to six decimal places.
        println!("x_{} = {:.6}", i, x);
    }
}
