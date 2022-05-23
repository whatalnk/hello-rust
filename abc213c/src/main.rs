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
    let hh = a.len();
    let ww = b.len();
    let mut dict = HashMap::new();
    for i in 0..hh {
        for j in 0..ww {
            dict.insert((*a[i], *b[j]), (i + 1, j + 1));
        }
    }
    for i in 0..n {
        let k = dict.get(&ab[i]).unwrap();
        println!("{} {}", k.0, k.1);
    }
}
