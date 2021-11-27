use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ll: [[i32; w]; h]
    }

    let mut h_sum = vec![0i32; w];
    let mut w_sum = vec![0i32; h];
    for i in 0..h {
        for j in 0..w {
            h_sum[j] += ll[i][j];
            w_sum[i] += ll[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{} ", w_sum[i] + h_sum[j] - ll[i][j]);
        }
    }

    println!();
}
