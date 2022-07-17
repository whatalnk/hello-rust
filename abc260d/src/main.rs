use proconio::input;
use std::collections::{BTreeSet, HashMap};
use std::ops::Bound::Included;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    let mut ans = vec![-1; n + 1];
    let mut hm = HashMap::<usize, Vec<usize>>::new();
    let mut bts = BTreeSet::<usize>::new();
    for i in 0..n {
        let c = p[i];
        let mut flag = false;
        for &elem in bts.range((Included(&c), Included(&n))) {
            hm.entry(elem).and_modify(|e| e.push(c));
            flag = true;
            bts.remove(&elem);
            if let Some((_, v)) = hm.remove_entry(&elem) {
                if v.len() == k {
                    for j in 0..v.len() {
                        ans[v[j]] = (i + 1) as isize;
                    }
                } else {
                    hm.insert(c, v);
                    bts.insert(c);
                }
            }
            break;
        }
        if !flag {
            if k == 1 {
                ans[c] = (i + 1) as isize;
            } else {
                let e = hm.entry(c).or_insert(Vec::new());
                e.push(c);
                bts.insert(c);
            }
        }
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
