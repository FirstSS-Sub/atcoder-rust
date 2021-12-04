use num_traits::Pow;
use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y1: f64,
        q: usize
    }

    for _ in 0..q {
        input! {e: f64}
        let theta = e / t * 2.0 * PI;
        let y2 = -l / 2.0 * theta.sin();
        let z = l / 2.0 * (1.0 - theta.cos());
        let length = ((x.pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
        println!("{}", (z / length).atan() * 180.0 / PI);
    }
}
