const MAX_ERROR: f64 = 1e-10;
const MAX_COMPUTE_WITH_GUESS_ITERATIONS: u32 = 50;

pub fn find_root_newton_raphson<Func, Deriv>(start: f64, f: Func, d: Deriv) -> f64
where
    Func: Fn(f64) -> f64,
    Deriv: Fn(f64) -> f64,
{
    // x[n + 1] = x[n] - f(x[n])/f'(x[n])

    let mut x = start;

    for _ in 0..MAX_COMPUTE_WITH_GUESS_ITERATIONS {
        let delta = f(x) / d(x);
        x -= delta;

        if delta.abs() < MAX_ERROR {
            return x - delta;
        }
    }

    f64::NAN
}

#[allow(dead_code)]
pub fn find_root_newton_raphson_with_default_deriv<Func>(start: f64, f: Func) -> f64
where
    Func: Fn(f64) -> f64,
{
    // deriv = (f(x + e) - f(x - e))/((x + e) - x)
    // multiply denominator by 2 for faster convergence

    // https://programmingpraxis.com/2012/01/13/excels-xirr-function/

    find_root_newton_raphson(start, &f, |x: f64| {
        (f(x + MAX_ERROR) - f(x - MAX_ERROR)) / (2.0 * MAX_ERROR)
    })
}