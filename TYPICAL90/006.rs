// https://atcoder.jp/contests/typical90/tasks/typical90_f

use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }
    let mut summary = vec![vec![std::usize::MAX; 26]; n + 1];
    let a = 'a' as usize;
    for (i, j) in (0..n).rev().cartesian_product(0..26) {
        let ch = s[i] as usize - a;
        summary[i][j] = if ch == j { i } else { summary[i + 1][j] };
    }
    let mut res = vec!['a'; k];
    let mut curr = 0;
    for i in 0..k {
        for j in 0..26 {
            if summary[curr][j] <= n - k + i {
                res[i] = (j + a) as u8 as char;
                curr = summary[curr][j] + 1;
                break;
            }
        }
    }
    println!("{}", res.iter().collect::<String>());
}
