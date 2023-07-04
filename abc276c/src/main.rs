use proconio::input;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included};

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    }
    let mut bts = BTreeSet::new();
    let mut ans = vec![];
    bts.insert(p[n - 1]);
    for i in 0..(n - 1) {
        if p[n - i - 1] > p[n - i - 2] {
            bts.insert(p[n - i - 2]);
        } else {
            let prev = *bts
                .range((Included(&0), Excluded(&p[n - i - 2])))
                .max()
                .unwrap();
            bts.insert(p[n - i - 2]);
            bts.remove(&prev);
            for pj in p.iter().take(n - i - 2) {
                ans.push(*pj);
            }
            ans.push(prev);
            break;
        }
    }
    let mut v = bts.iter().collect::<Vec<&i64>>();
    v.reverse();
    for vi in &v {
        ans.push(**vi);
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
