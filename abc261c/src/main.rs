use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        if hm.contains_key(&s[i]) {
            println!("{}({})", s[i], hm.get(&s[i]).unwrap());
            hm.entry(&s[i]).and_modify(|e| *e += 1);
        } else {
            println!("{}", s[i]);
            hm.insert(&s[i], 1);
        }
    }
}
