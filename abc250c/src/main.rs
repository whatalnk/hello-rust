use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q]
    }
    let mut b = vec![];
    for i in 1..=n {
        b.push(i);
    }
    let mut hm = HashMap::new();
    for i in 1..=n {
        hm.insert(i, i);
    }
    for i in x {
        let l = hm.remove(&i).unwrap();
        let r = if l == n { l - 1 } else { l + 1 };
        b.swap(l - 1, r - 1);
        hm.insert(b[l - 1], l);
        hm.insert(b[r - 1], r);
    }
    println!(
        "{}",
        b.iter()
            .map(|ii| ii.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
