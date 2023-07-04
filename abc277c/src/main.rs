use proconio::input;
use std::collections::{BTreeSet, HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut hm = HashMap::<usize, Vec<usize>>::new();
    for abi in ab.iter().take(n) {
        let (a, b) = abi;
        let e = hm.entry(*a).or_insert_with(|| vec![]);
        e.push(*b);
        let e = hm.entry(*b).or_insert_with(|| vec![]);
        e.push(*a);
    }
    let mut que = VecDeque::new();
    que.push_back(1);
    let mut bts = BTreeSet::new();
    bts.insert(1);
    while !que.is_empty() {
        if let Some(v) = que.pop_front() {
            if let Some(g) = hm.get(&v) {
                for gi in &(*g) {
                    if !bts.contains(gi) {
                        que.push_back(*gi);
                        bts.insert(*gi);
                    }
                }
            }
        }
    }
    println!("{}", bts.iter().last().unwrap());
}
