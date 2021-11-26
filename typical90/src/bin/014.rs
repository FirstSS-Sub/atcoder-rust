use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n]
    }
    a.sort_unstable();
    b.sort_unstable();

    let mut ans = 0;
    for i in 0..n {
        ans += (a[i] - b[i]).abs();
    }
    println!("{}", ans);
}
