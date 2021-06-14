// https://atcoder.jp/contests/abc076/tasks/abc076_c

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _s: Chars,
        t: Chars
    }
    let n = _s.len();
    let m = t.len();
    if n < m {
        println!("{}", "UNRESTORABLE");
        return;
    }
    if let Some(p) = (0..=(n - m)).rev().find(|i| {
        (*i..(i + m))
            .zip(0..m)
            .all(|(x, y)| _s[x] == t[y] || _s[x] == '?')
    }) {
        let res = _s[..p]
            .iter()
            .chain(t.iter())
            .chain(_s[(p + m)..].iter())
            .collect::<String>()
            .replace("?", "a");
        println!("{}", res);
    } else {
        println!("{}", "UNRESTORABLE");
    }
}
