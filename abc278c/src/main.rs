use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        _n: usize,
        q: usize,
        tab: [(i64, usize, usize); q],
    }
    let mut s = BTreeSet::<(usize, usize)>::new();
    for tabi in tab.iter().take(q) {
        let (t, a, b) = tabi;
        if t == &1 {
            s.insert((*a, *b));
        } else if t == &2 {
            s.remove(&(*a, *b));
        } else if s.contains(&(*a, *b)) && s.contains(&(*b, *a)) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
