// https://atcoder.jp/contests/typical90/tasks/typical90_c

use petgraph::{algo::dijkstra, graph::{node_index, UnGraph}};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1]
    }
    let graph = UnGraph::<usize, usize, usize>::from_edges(ab);
    let first = dijkstra(&graph, node_index(0), None, |_| 1);
    let target = first.iter().max_by_key(|(_, dist)| *dist).unwrap().0;
    let second = dijkstra(&graph, *target, None, |_| 1);
    let res = second.iter().max_by_key(|(_, dist)| *dist).unwrap().1;
    println!("{}", res + 1);
}