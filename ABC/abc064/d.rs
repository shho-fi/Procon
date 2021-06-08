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
    left = std::cmp::min(left, 0);
    left = num::Signed::abs(&left);
    let mut right = left;
    for ch in s.chars() {
        right += if ch == '(' { 1 } else { -1 };
    }
    let pre = vec!['('; left as usize];
    let suf = vec![')'; right as usize].into_iter();
    let res: String = pre
        .into_iter()
        .chain(s.chars().into_iter())
        .chain(suf.into_iter())
        .collect();
    println!("{}", res);
}