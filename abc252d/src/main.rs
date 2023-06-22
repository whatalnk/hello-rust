use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut hm = HashMap::new();
    for ai in a.iter().take(n) {
        let e = hm.entry(*ai).or_insert(0);
        *e += 1;
    }
    let mut keys: Vec<i64> = hm.keys().cloned().collect();
    keys.sort();
    let mut cum = vec![0; keys.len()];
    for i in 1..keys.len() {
        cum[i] = cum[i - 1] + hm.get(&keys[i - 1]).cloned().unwrap();
    }
    let mut ans = 0;
    for aj in a.iter().take(n) {
        let cur = keys.binary_search(aj).unwrap();
        if cur > 0 && cur < keys.len() - 1 {
            let ii = cum[cur];
            let kk = n - cum[cur + 1];
            ans += ii * kk;
        }
    }
    println!("{}", ans);
}
