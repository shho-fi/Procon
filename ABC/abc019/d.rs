// https://atcoder.jp/contests/abc019/tasks/abc019_4

fn main() {
    let n = scan_i32();
    let mut target = (0, 0);
    for i in 2..=n {
        let dist = query(1, i);
        if target.1 < dist {
            target = (i, dist);
        }
    }
    let mut res = 0;
    for i in 1..=n {
        let dist = query(target.0, i);
        res = std::cmp::max(res, dist);
    }
    println!("! {}", res);
}
fn query(from: i32, to: i32) -> i32 {
    println!("? {} {}", from, to);
    scan_i32()
}
fn scan_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<i32>().unwrap()
}
