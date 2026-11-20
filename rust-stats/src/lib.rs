//! A small numeric library used to exercise twinwrap. Nothing here knows
//! about Python, JavaScript or C.

pub fn add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

pub fn mean(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return f64::NAN;
    }
    // Pairwise sum, so the result does not depend on how a caller batches.
    fn sum(xs: &[f64]) -> f64 {
        match xs.len() {
            0 => 0.0,
            1 => xs[0],
            n => sum(&xs[..n / 2]) + sum(&xs[n / 2..]),
        }
    }
    sum(xs) / xs.len() as f64
}

pub fn scale(xs: &[f64], k: f64) -> Vec<f64> {
    xs.iter().map(|x| x * k).collect()
}

pub fn cumsum(xs: &[i64]) -> Vec<i64> {
    let mut acc = 0i64;
    xs.iter().map(|x| { acc = acc.wrapping_add(*x); acc }).collect()
}

pub fn to_f32(xs: &[f64]) -> Vec<f32> {
    xs.iter().map(|x| *x as f32).collect()
}

pub fn is_sorted(xs: &[f64]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}

pub fn shout(s: &str) -> String {
    format!("{}!", s.to_uppercase())
}

pub fn word_count(s: &str) -> u32 {
    s.split_whitespace().count() as u32
}

pub fn checked_div(a: i64, b: i64) -> i64 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}
