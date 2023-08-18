use std::{collections::HashMap, println, vec};

use proconio::input;

fn main() {
    input!(n: usize);
    let mut hm = HashMap::new();
    let mut c = vec![];
    for i in 0..n {
        input!(ci: usize);
        c.push(ci);
        input!(ai: [i64; ci]);
        for aii in ai {
            let e = hm.entry(aii).or_insert_with(Vec::new);
            e.push(i);
        }
    }
    input!(x: i64);
    if let Some(cand) = hm.get(&x) {
        if let Some(lmin) = cand.iter().map(|i| c[*i]).min() {
            let ans = cand
                .iter()
                .filter(|i| c[**i] == lmin)
                .map(|i| (i + 1).to_string())
                .collect::<Vec<_>>();
            println!("{}", ans.len());
            println!("{}", ans.join(" "));
        }
    } else {
        println!("0");
    }
}
