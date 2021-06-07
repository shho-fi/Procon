// https://atcoder.jp/contests/abc190/tasks/abc190_c

use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        k: usize,
        cd: [[usize; 2]; k]
    }
    let mut res = 0;
    for bit in 0..(1 << k) {
        let mut dishes = vec![false; n + 1];
        for i in 0..k {
            dishes[cd[i][bit >> i & 1]] = true;
        }
        res = std::cmp::max(res, ab.iter().filter(|(a, b)|dishes[*a] && dishes[*b]).count());
    }
    println!("{}", res);
}