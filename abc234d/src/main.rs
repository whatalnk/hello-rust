use proconio::input;
use std::collections::BTreeSet;
use std::ops::Bound::Included;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [i64; n]
    }
    let mut bts = BTreeSet::new();
    let mut vk: i64 = n as i64;
    for i in 0..k {
        vk = vk.min(p[i]);
        bts.insert(p[i]);
    }
    println!("{}", vk);
    for i in k..n {
        bts.insert(p[i]);
        if vk < p[i] {
            let mut it = bts.range((Included(&(vk + 1)), Included(&p[i])));
            vk = *it.next().unwrap();
        }
        println!("{}", vk);
    }
}
