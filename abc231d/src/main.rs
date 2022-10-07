use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut v = vec![HashSet::new(); n + 1];
    for i in 0..m {
        let (a, b) = ab[i];
        v[a].insert(b);
        v[b].insert(a);
        if v[a].len() > 2 || v[b].len() > 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
