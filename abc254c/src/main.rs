use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n]
    }
    let mut m = vec![BinaryHeap::new(); k];
    for i in 0..n {
        m[(i + 1) % k].push(Reverse(a[i]));
    }
    let mut sorted = vec![];
    for i in 0..n {
        if let Some(Reverse(v)) = m[(i + 1) % k].pop() {
            sorted.push(v);
        }
    }
    for i in 0..(n - 1) {
        if sorted[i] > sorted[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
