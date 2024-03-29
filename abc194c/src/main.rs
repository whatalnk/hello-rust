use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n]
    }
    let mut hm = HashMap::new();
    for ai in a.iter() {
        let e = hm.entry(ai).or_insert(0);
        *e += 1;
    }
    let ks: Vec<&i64> = hm.keys().cloned().collect();
    let l = ks.len();
    let mut ans = 0;
    for i in 0..l {
        for j in (i + 1)..l {
            ans += (ks[i] - ks[j]).pow(2) * hm[ks[i]] * hm[ks[j]];
        }
    }
    println!("{}", ans);
}
