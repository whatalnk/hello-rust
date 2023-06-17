use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n]
    }
    let mut hm = HashMap::new();
    for (i, ai) in a.iter().enumerate().take(n) {
        let e = hm.entry(ai).or_insert_with(|| vec![]);
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
