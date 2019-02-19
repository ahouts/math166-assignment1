use crate::function::Function;
use std::error::Error;
use std::fmt;

pub const MAX_NEWTONS_ITERATIONS: u64 = 100;

pub trait Continuous: Function {
    fn derivative(&self, x: f64) -> f64;
    fn newtons(&self, x0: f64, accuracy: f64) -> Result<(f64, u64), NewtonError> {
        let mut n_iter = 0;
        let mut x = x0;
        loop {
            if n_iter > MAX_NEWTONS_ITERATIONS {
                return Err(NewtonError::MaxIterReached);
            }
            let x1 = x - self.eval(x) / self.derivative(x);
            n_iter += 1;
            if f64::abs(x1 - x) < accuracy {
                return Ok((x1, n_iter));
            }
            x = x1;
        }
    }
}

// literal continuous function, implemented using rust functions
// representing evaluation and evaluating the derivative
pub struct ContFuncLit {
    f: Box<Fn(f64) -> f64 + Sync>,
    df: Box<Fn(f64) -> f64 + Sync>,
}

impl ContFuncLit {
    pub fn new(f: Box<Fn(f64) -> f64 + Sync>, df: Box<Fn(f64) -> f64 + Sync>) -> Self {
        ContFuncLit { f, df }
    }
}

impl Function for ContFuncLit {
    fn eval(&self, x: f64) -> f64 {
        (*self.f)(x)
    }
}

impl Continuous for ContFuncLit {
    fn derivative(&self, x: f64) -> f64 {
        (*self.df)(x)
    }
}

#[derive(Debug)]
pub enum NewtonError {
    MaxIterReached,
}

impl Error for NewtonError {}

impl fmt::Display for NewtonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "maximum number of iterations reached")
    }
}
