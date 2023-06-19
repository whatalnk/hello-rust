use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64
    }
    let dx = [1, 2, 2, 1, -1, -2, -2, -1];
    let dy = [2, 1, -1, -2, -2, -1, 1, 2];
    let mut ps1 = HashSet::new();
    let mut ps2 = HashSet::new();
    for i in 0..8 {
        ps1.insert((x1 + dx[i], y1 + dy[i]));
        ps2.insert((x2 + dx[i], y2 + dy[i]));
    }
    let intersection: HashSet<_> = ps1.intersection(&ps2).collect();
    if !intersection.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
