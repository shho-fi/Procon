// https://atcoder.jp/contests/abc023/tasks/abc023_d

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        hs: [(i64, i64); n]
    }
    let mut left = -1;
    let mut right = 1000000000 + n as i64 * 1000000000;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(&hs, n, mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}
fn is_ok(hs: &[(i64, i64)], n: usize, mid: i64) -> bool {
    let mut bucket = vec![0; n];
    for (h, s) in hs {
        if mid < *h {
            return false;
        }
        let t = ((mid - h) / s) as usize;
        bucket[std::cmp::min(n - 1, t)] += 1;
    }
    if bucket[0] > 1 {
        return false;
    }
    for i in 1..n {
        bucket[i] += bucket[i - 1];
        if bucket[i] > i + 1 {
            return false;
        }
    }
    true
}