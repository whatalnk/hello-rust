use proconio::input;
use std::collections::{BTreeSet, HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut hm = HashMap::<usize, Vec<usize>>::new();
    for i in 0..n {
        let (a, b) = ab[i];
        let e = hm.entry(a).or_insert(vec![]);
        e.push(b);
        let e = hm.entry(b).or_insert(vec![]);
        e.push(a);
    }
    let mut que = VecDeque::new();
    que.push_back(1);
    let mut bts = BTreeSet::new();
    bts.insert(1);
    while que.len() != 0 {
        if let Some(v) = que.pop_front() {
            if let Some(g) = hm.get(&v) {
                for i in 0..g.len() {
                    if !bts.contains(&g[i]) {
                        que.push_back(g[i]);
                        bts.insert(g[i]);
                    }
                }
            }
        }
    }
    println!("{}", bts.iter().last().unwrap());
}
