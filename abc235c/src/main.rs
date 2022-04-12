use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n]
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        let e = hm.entry(a[i]).or_insert(vec![]);
        e.push(i + 1);
    }
    for _ in 0..q {
        input! {
            x: i64,
            k: usize
        }
        if hm.contains_key(&x) {
            let v = hm.get(&x).unwrap();
            if k > v.len() {
                println!("-1");
            } else {
                println!("{}", v[k - 1]);
            }
        } else {
            println!("-1");
        }
    }
}
