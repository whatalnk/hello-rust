use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }
    let mut vd = VecDeque::new();
    for i in 0..n {
        vd.push_back(a[i].to_string());
    }
    for _ in 0..k {
        vd.pop_front();
        vd.push_back("0".to_string());
    }
    println!("{}", vd.iter().cloned().collect::<Vec<String>>().join(" "));
}
