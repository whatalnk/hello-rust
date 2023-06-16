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
    for pi in p.iter().take(k) {
        vk = vk.min(*pi);
        bts.insert(*pi);
    }
    println!("{}", vk);
    for pi in p.iter().take(n).skip(k) {
        bts.insert(*pi);
        if &vk < pi {
            let mut it = bts.range((Included(&(vk + 1)), Included(pi)));
            vk = *it.next().unwrap();
        }
        println!("{}", vk);
    }
}
