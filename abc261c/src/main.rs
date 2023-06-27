use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut hm = HashMap::new();
    for si in s.iter().take(n) {
        if hm.contains_key(si) {
            println!("{}({})", si, hm.get(&si).unwrap());
            hm.entry(si).and_modify(|e| *e += 1);
        } else {
            println!("{}", si);
            hm.insert(si, 1);
        }
    }
}
