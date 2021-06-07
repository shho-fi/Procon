// https://atcoder.jp/contests/typical90/tasks/typical90_a

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        l: i32,
        k: i32,
        a: [i32; n]
    }
    let mut left = -1;
    let mut right = l + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(&a, l, k, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
fn is_ok(a: &Vec<i32>, l: i32, k: i32, mid: i32) -> bool {
    let mut prev = 0;
    let mut count = 0;
    for curr in a {
        if curr - prev >= mid && l - curr >= mid {
            prev = *curr;
            count += 1;
        }
    }
    count >= k
}
