use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize}
    let mut c1: Vec<u32> = vec![0];
    let mut c2: Vec<u32> = vec![0];
    for i in 0..n {
        input! {c: usize, p: u32}
        match c {
            1 => {
                c1.push(c1[i] + p);
                c2.push(c2[i]);
            }
            2 => {
                c1.push(c1[i]);
                c2.push(c2[i] + p);
            }
            _ => {}
        }
    }

    input! {q: usize}
    for _ in 0..q {
        input! {l:usize, r:usize}
        println!("{} {}", c1[r] - c1[l - 1], c2[r] - c2[l - 1]);
    }
}
