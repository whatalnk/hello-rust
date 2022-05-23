use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize, usize); n]
    }
    let mut sa = HashSet::new();
    let mut sb = HashSet::new();
    for i in 0..n {
        sa.insert(ab[i].0);
        sb.insert(ab[i].1);
    }
    let mut a: Vec<_> = sa.iter().collect();
    let mut b: Vec<_> = sb.iter().collect();
    a.sort();
    b.sort();
    let mut ai = HashMap::new();
    let mut bi = HashMap::new();
    for i in 0..a.len() {
        ai.insert(*a[i], i + 1);
    }
    for i in 0..b.len() {
        bi.insert(*b[i], i + 1);
    }
    for i in 0..n {
        println!(
            "{} {}",
            ai.get(&ab[i].0).unwrap(),
            bi.get(&ab[i].1).unwrap()
        );
    }
}
