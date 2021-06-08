// https://atcoder.jp/contests/abc064/tasks/abc064_d

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: i32,
        s: String
    }
    let mut left = 0;
    let mut count = 0;
    for ch in s.chars() {
        count += if ch == '(' { 1 } else { -1 };
        left = std::cmp::min(left, count);
    }
    left = num::Signed::abs(&left);
    let right = left + count;
    let pre = vec!['('; left as usize];
    let suf = vec![')'; right as usize];
    let res: String = pre
        .into_iter()
        .chain(s.chars().into_iter())
        .chain(suf.into_iter())
        .collect();
    println!("{}", res);
}