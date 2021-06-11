// https://atcoder.jp/contests/abc129/tasks/abc129_d

use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut up = vec![vec![0; w]; h];
    let mut down = vec![vec![0; w]; h];
    let mut left = vec![vec![0; w]; h];
    let mut right = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            up[i][j] = if i == 0 { 1 } else { up[i - 1][j] + 1 };
            left[i][j] = if j == 0 { 1 } else { left[i][j - 1] + 1 };
        }
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if s[i][j] == '#' {
                continue;
            }
            down[i][j] = if i == h - 1 { 1 } else { down[i + 1][j] + 1 };
            right[i][j] = if j == w - 1 { 1 } else { right[i][j + 1] + 1 };
        }
    }
    let res = (0..h)
        .cartesian_product(0..w)
        .filter(|(i, j)| s[*i][*j] != '#')
        .map(|(i, j)| up[i][j] + down[i][j] + left[i][j] + right[i][j] - 3)
        .max()
        .unwrap();
    println!("{}", res);
}
