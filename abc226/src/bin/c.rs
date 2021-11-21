use std::vec;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut t_list: Vec<u64> = vec![];
    let mut waza_list :Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        input! {
            t: u64,
            waza: [usize]
        }
        t_list.push(t);
        waza_list.push(waza);
    }

    let mut ans: u64 = 0;
    let mut flag = vec![false; n];
    flag[n-1] = true;
    for i in (0..n).rev() {
        if flag[i] {
            ans += t_list[i];
            for v in waza_list[i].iter() {
                flag[v-1] = true;
            }
        }
    }

    println!("{}", ans);
}
