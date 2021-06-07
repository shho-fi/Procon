// https://atcoder.jp/contests/typical90/tasks/typical90_b

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    for bit in 0..(1 << n) {
        let bin = format!("{:0width$b}", bit, width = n);
        if is_ok(&bin) {
            let s = bin.replace("0", "(").replace("1", ")");
            println!("{}", s);
        }
    }
}
fn is_ok(s: &String) -> bool {
    let mut count = 0;
    for ch in s.chars() {
        count += if ch == '0' { 1 } else { -1 };
        if count < 0 {
            return false;
        }
    }
    count == 0
}
