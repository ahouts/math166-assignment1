mod continuous;
mod function;
mod poly;

use crate::continuous::{ContFuncLit, Continuous};
use crate::function::Function;
use crate::poly::Poly;

fn main() {
    println!();
    println!("##################################################");
    println!("# Problem #1                                     #");
    println!("##################################################");
    println!();
    for p_num in 1..10 {
        let p = 0.05 + 0.05 * p_num as f64;
        let f = ContFuncLit::new(
            Box::new(move |x: f64| f64::cos(x) - 0.8 + p * x.powi(2)),
            Box::new(move |x: f64| -f64::sin(x) + p * 2.0 * x),
        );
        println!("p = {:.2}", p);
        let sign_changes = f.find_sign_changes(-5.0, 5.0, 1e-3);
        if sign_changes.is_empty() {
            println!("  no roots found");
            continue;
        }
        for (x0, x1) in sign_changes {
            println!("  finding root in range ({:.5}, {:.5})", x0, x1);
            let (res, n_iter) = f.bisection(x0, x1, 1e-10);
            println!("    number of bisection iterations = {}", n_iter);
            println!("    res = {:.10}", res);
            let res = match f.newtons(res, 1e-50) {
                Ok(r) => r,
                Err(e) => {
                    println!("    newtons method took too many iterations, use bisection for final evaluation");
                    let (res, n_iter) = f.bisection(x0, x1, 1e-15);
                    println!("    number of bisection iterations = {}", n_iter);
                    println!("    res = {:.15}", res);
                    println!("    f(res) = {:.15}", f.eval(res));
                    continue;
                }
            };
            let (res, n_iter) = res;
            println!("    number of newton iterations = {}", n_iter);
            println!("    res = {:.50}", res);
            println!("    f(res) = {:.50}", f.eval(res));
            println!();
        }
    }

    println!();
    println!("##################################################");
    println!("# Problem #2a,b                                  #");
    println!("##################################################");
    println!();

    let b_vals = vec![1.0, 1e10, 1e-10, 250.0, 1e-5];
    let b_vals2 = b_vals.clone();
    for b in b_vals {
        let f = ContFuncLit::new(
            Box::new(move |x: f64| f64::sin(x) - b / x * f64::cos(x)),
            Box::new(move |x: f64| f64::cos(x) + b / x * f64::sin(x) + b / x.powi(2) * f64::cos(x)),
        );
        println!("b = {:.10}", b);
        // either sin(x) or b/x*cos(x) will dominate the other, the magnitude of b determines which one it will be
        let dominant_term = f64::max(b, 1.0);
        let sign_changes = f.find_sign_changes(0.0, 15.0 * dominant_term, 0.1 * dominant_term);
        if sign_changes.is_empty() {
            println!("  no roots found");
            continue;
        }
        for (x0, x1) in sign_changes.into_iter().take(3) {
            println!("  finding root in range ({:.1}, {:.1})", x0, x1);
            let (res, n_iter) = f.bisection(x0, x1, 1e-10 * dominant_term);
            println!("    number of bisection iterations = {}", n_iter);
            println!("    res = {:.10}", res);
            let res = match f.newtons(res, 1e-50) {
                Ok(r) => r,
                Err(e) => {
                    println!("    error while calculating newtons: {}", e);
                    continue;
                }
            };
            let (res, n_iter) = res;
            println!("    number of newton iterations = {}", n_iter);
            println!("    res = {:.50}", res);
            println!("    f(res) = {:.50}", f.eval(res));
            println!();
        }
    }

    println!();
    println!("##################################################");
    println!("# Problem #2c                                    #");
    println!("##################################################");
    println!();

    for b in b_vals2 {
        let f = ContFuncLit::new(
            Box::new(move |x: f64| f64::tan(x) - b / x),
            Box::new(move |x: f64| (1.0 / f64::cos(x)).powi(2) + b / x.powi(2)),
        );
        println!("b = {:.10}", b);
        // either sin(x) or b/x*cos(x) will dominate the other, the magnitude of b determines which one it will be
        let dominant_term = f64::max(b, 1.0);
        let sign_changes = f.find_sign_changes(0.0, 20.0 * dominant_term, 0.1 * dominant_term);
        if sign_changes.is_empty() {
            println!("  no roots found");
            continue;
        }
        for (x0, x1) in sign_changes.into_iter().take(3) {
            println!("  finding root in range ({:.20}, {:.20})", x0, x1);
            let (res, n_iter) = f.bisection(x0, x1, 1e-10 * dominant_term);
            println!("    number of bisection iterations = {}", n_iter);
            println!("    res = {:.10}", res);
            let res = match f.newtons(res, 1e-50) {
                Ok(r) => r,
                Err(e) => {
                    println!("    error while calculating newtons: {}", e);
                    continue;
                }
            };
            let (res, n_iter) = res;
            println!("    number of newton iterations = {}", n_iter);
            println!("    res = {:.50}", res);
            println!("    f(res) = {:.50}", f.eval(res));
            println!();
        }
    }

    println!();
    println!("##################################################");
    println!("# Problem #3                                     #");
    println!("##################################################");
    println!();

    let p: Poly = vec![0.3136, -1.680, 3.37, -3.0, 1.0].into_iter().collect();

    //    println!("try to find root bounds by locating positive/negative x values");
    //    for step_size in vec![1e-5, 1e-6, 1e-7, 1e-8, 1e-9].into_iter() {
    //        println!("  step size = {}", step_size);
    //        for (x0, x1) in p.find_sign_changes(0.5, 1.0, step_size) {
    //            println!("    {}, {}", x0, x1);
    //        }
    //    }

    println!("try newtons on range");
    for (x0, x1) in p.find_sign_changes(0.5, 1.0, 1e-8) {
        println!("  finding root in range ({:.8}, {:.8})", x0, x1);
        let (res, n_iter) = p.bisection(x0, x1, 1e-13);
        println!("    number of bisection iterations = {}", n_iter);
        println!("    res = {:.10}", res);
        let res = match p.newtons(res, 1e-50) {
            Ok(r) => r,
            Err(e) => {
                println!("    error while calculating newtons: {}", e);
                continue;
            }
        };
        let (res, n_iter) = res;
        println!("    number of newton iterations = {}", n_iter);
        println!("    res = {:.50}", res);
        println!("    p(res) = {:.50}", p.eval(res));
        println!();
    }

    println!();
    println!("##################################################");
    println!("# Problem #4                                     #");
    println!("##################################################");
    println!();

    let p: Poly = vec![48.5625, -156.6, 212.6625, -151.85, 59.5, -12.1, 1.0]
        .into_iter()
        .collect();
    println!("check if the sign of f(x) changes on (2, 2.2), 1e-9 increments");
    assert!(p.find_sign_changes(2.0, 2.2, 1e-9).is_empty());
    println!("the sign of f(x) does not change on (2, 2.2), 1e-9 increments");
    println!("check if newtons method converges");
    assert!(p.newtons(2.1, 1e-30).is_err());
    println!("newtons method does not converge");

    println!();
    println!("##################################################");
    println!("# Problem #5                                     #");
    println!("##################################################");
    println!();

    let p: Poly = vec![4.37, -5.75, 1.5, 1.0].into_iter().collect();
    let sign_changes = p.find_sign_changes(-10.0, 10.0, 0.1);
    if sign_changes.is_empty() {
        println!("  no roots found");
    } else {
        for (x0, x1) in sign_changes {
            println!("  finding root in range ({:.1}, {:.1})", x0, x1);
            let (res, n_iter) = p.bisection(x0, x1, 1e-10);
            println!("    number of bisection iterations = {}", n_iter);
            println!("    res = {:.10}", res);
            let res = match p.newtons(res, 1e-50) {
                Ok(r) => r,
                Err(e) => {
                    println!("    error while calculating newtons: {}", e);
                    continue;
                }
            };
            let (res, n_iter) = res;
            println!("    number of newton iterations = {}", n_iter);
            println!("    res = {:.50}", res);
            println!("    f(res) = {:.50}", p.eval(res));
            println!();
        }
    }
}
