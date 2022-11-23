use proconio::input;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        tab: [(i64, usize, usize); q],
    }
    let mut m = BTreeMap::<usize, BTreeSet<usize>>::new();
    for i in 0..q {
        let (t, a, b) = tab[i];
        if t == 1 {
            let e = m.entry(a).or_insert(BTreeSet::<usize>::new());
            e.insert(b);
        } else if t == 2 {
            let e = m.entry(a).or_insert(BTreeSet::<usize>::new());
            e.remove(&b);
        } else {
            let mut ans = true;
            if let Some(e) = m.get(&a) {
                if !e.contains(&b) {
                    ans = false;
                }
            } else {
                ans = false;
            }
            if let Some(e) = m.get(&b) {
                if !e.contains(&a) {
                    ans = false;
                }
            } else {
                ans = false;
            }
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
