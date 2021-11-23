use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize
    }

    a.sort_unstable();
    for _ in 0..q {
        let mut l: usize = 0;
        let mut r = n - 1;
        let mut key = (l + r) / 2;
        input! {b: i32}
        loop {
            if a[key] > b {
                if r - l <= 1 {
                    if (a[l] - b).abs() <= (a[key] - b).abs() {
                        println!("{}", (a[l] - b).abs());
                    } else {
                        println!("{}", (a[key] - b).abs());
                    }
                    break;
                }
                r = key;
                key = (l + r) / 2;
            } else {
                if r - l <= 1 {
                    if (a[r] - b).abs() <= (a[key] - b).abs() {
                        println!("{}", (a[r] - b).abs());
                    } else {
                        println!("{}", (a[key] - b).abs());
                    }
                    break;
                }
                l = key;
                key = (l + r) / 2;
            }
        }
    }
}
