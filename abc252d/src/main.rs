use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        let e = hm.entry(a[i]).or_insert(0);
        *e += 1;
    }
    let mut keys: Vec<i64> = hm.keys().cloned().collect();
    keys.sort();
    let mut cum = vec![0; keys.len()];
    for i in 1..keys.len() {
        cum[i] = cum[i - 1] + hm.get(&keys[i - 1]).cloned().unwrap();
    }
    let mut ans = 0;
    for j in 0..n {
        let cur = keys.binary_search(&a[j]).unwrap();
        if cur > 0 && cur < keys.len() - 1 {
            let ii = cum[cur];
            let kk = n - cum[cur + 1];
            ans += ii * kk;
        }
    }
    println!("{}", ans);
}
