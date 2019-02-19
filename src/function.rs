use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::mem::swap;

pub trait Function: Sync {
    fn eval(&self, x: f64) -> f64;

    fn bisection(&self, mut x0: f64, mut x1: f64, accuracy: f64) -> (f64, u64) {
        let mut n_iter = 0;
        if self.eval(x1) < self.eval(x0) {
            swap(&mut x1, &mut x0);
        }
        assert!(self.eval(x0) <= 0.0);
        assert!(self.eval(x1) >= 0.0);
        loop {
            let n = (x1 + x0) / 2.0;
            let yn = self.eval(n);
            if yn > 0.0 {
                x1 = n;
            } else {
                x0 = n;
            }
            n_iter += 1;
            if f64::abs(x1 - x0) < accuracy {
                return (x1, n_iter);
            }
        }
    }

    fn find_sign_changes(&self, start: f64, end: f64, step_size: f64) -> Vec<(f64, f64)> {
        let num_steps = ((end - start) / step_size).ceil() as u64;
        (1..num_steps)
            .into_par_iter()
            .filter_map(|pos| {
                let x0 = start + (step_size * (pos - 1) as f64);
                let y0 = self.eval(x0);
                let x1 = x0 + step_size;
                let y1 = self.eval(x1);
                if ((y0 <= 0.0 && y1 >= 0.0) || (y0 >= 0.0 && y1 <= 0.0))
                    // make sure we haven't found pos&neg infinity
                    && self.eval((x0 + x1) / 2.0).abs() < 1.0
                {
                    Some((x0, x1))
                } else {
                    None
                }
            })
            .collect()
    }
}
