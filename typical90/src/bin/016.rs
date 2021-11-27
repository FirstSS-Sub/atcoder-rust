use nalgebra::min;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let lim = 9999;
    let mut ans = 10000;
    for x in 0..=lim {
        for y in 0..=lim {
            if x + y > lim || n - a * x - b * y < 0 {
                break;
            }
            let z = (n - a * x - b * y) / c;
            if a * x + b * y + c * z == n {
                ans = min(ans, x + y + z);
            }
        }
    }

    println!("{}", ans);
}
