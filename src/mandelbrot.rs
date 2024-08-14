use num::{complex::ComplexFloat, Complex};

pub const MAX_ITERATIONS: usize = 1000;

#[must_use]
pub fn get_iterations(x: f64, y: f64) -> usize {
    let mut z = Complex::from(0.0);
    let c = Complex::new(x, y);

    let mut iterations = 0;

    while iterations < MAX_ITERATIONS {
        z = z * z + c;

        if z.abs() > 2.0 {
            break;
        }

        iterations += 1;
    }

    iterations
}
