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
    for item in ab.iter() {
        sa.insert(item.0);
        sb.insert(item.1);
    }
    let mut a: Vec<_> = sa.iter().collect();
    let mut b: Vec<_> = sb.iter().collect();
    a.sort();
    b.sort();
    let mut ai = HashMap::new();
    let mut bi = HashMap::new();
    for (i, item) in a.iter().enumerate() {
        ai.insert(*item, i + 1);
    }
    for (i, item) in b.iter().enumerate() {
        bi.insert(*item, i + 1);
    }
    for item in ab.iter() {
        println!("{} {}", ai.get(&item.0).unwrap(), bi.get(&item.1).unwrap());
    }
}
