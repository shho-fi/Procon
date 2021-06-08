// https://atcoder.jp/contests/abc197/tasks/abc197_c

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut res = std::i64::MAX;
    for bit in 0..(1 << n) {
        let mut xor = 0;
        let mut or = 0;
        for i in 0..n {
            or |= a[i];
            if bit >> i & 1 == 1 {
                xor ^= or;
                or = 0;
            }
        }
        xor ^= or;
        res = std::cmp::min(res, xor);
    }
    println!("{}", res);
}