// https://atcoder.jp/contests/typical90/tasks/typical90_d

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h]
    }
    let sums_r = (0..h)
        .into_iter()
        .map(|i| a[i].iter().sum())
        .collect::<Vec<i32>>();
    let sums_c = (0..w)
        .into_iter()
        .map(|i| a.iter().map(|x| x[i]).sum())
        .collect::<Vec<i32>>();
    for i in 0..h {
        for j in 0..(w - 1) {
            print!("{} ", sums_r[i] + sums_c[j] - a[i][j]);
        }
        println!("{}", sums_r[i] + sums_c[w - 1] - a[i][w - 1]);
    }
}