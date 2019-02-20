use crate::continuous::Continuous;
use crate::function::Function;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Poly {
    // coefficients of the polynomial, starting with pow=0 at index 0, pow=1 at index 1, etc...
    coef: Vec<f64>,
    // coefficients of the derivative of the polynomial
    dx_coef: Vec<f64>,
}

impl Poly {
    pub fn dx(coef: &[f64]) -> Vec<f64> {
        coef.iter()
            .enumerate()
            .skip(1)
            .map(|(pow, coef)| *coef * pow as f64)
            .collect()
    }
}

impl Function for Poly {
    fn eval(&self, x: f64) -> f64 {
        let mut i = self.coef.iter().rev();
        let mut res = *i.next().unwrap();
        for coef in i {
            res = res * x + *coef;
        }
        res
    }
}

impl Continuous for Poly {
    fn derivative(&self, x: f64) -> f64 {
        let mut i = self.dx_coef.iter().rev();
        let mut res = *i.next().unwrap();
        for coef in i {
            res = res * x + *coef;
        }
        res
    }
}

impl FromIterator<f64> for Poly {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let coef: Vec<f64> = iter.into_iter().collect();
        let dx_coef = Self::dx(coef.as_slice());
        Poly { coef, dx_coef }
    }
}
