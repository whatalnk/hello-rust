use proconio::input;
use std::collections::BTreeSet;

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
            bts.insert(p[n - i - 2]);
            bts.remove(&(p[n - i - 2] - 1));
            for j in 0..(n - i - 2) {
                ans.push(p[j]);
            }
            ans.push(p[n - i - 2] - 1);
            break;
        }
    }
    let mut v = bts.iter().collect::<Vec<&i64>>();
    v.reverse();
    for i in 0..v.len() {
        ans.push(*v[i]);
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
