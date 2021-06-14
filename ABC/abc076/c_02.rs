// https://atcoder.jp/contests/abc076/tasks/abc076_c

use proconio::{fastout, input};
use regex::Regex;

#[fastout]
fn main() {
    input! {
        _s: String,
        t: String
    }
    let n = _s.len();
    let m = t.len();
    if n < m {
        println!("{}", "UNRESTORABLE");
        return;
    }
    let r = _s.replace("?", ".");
    if let Some(p) = (0..=(n - m)).rev().find(|i| {
        let regex = Regex::new(&r[*i..(i + m)]).unwrap();
        regex.is_match(&t)
    }) {
        let s = _s.replace("?", "a");
        println!("{}{}{}", &s[..p], t, &s[p + m..]);
    } else {
        println!("{}", "UNRESTORABLE");
    }
}
