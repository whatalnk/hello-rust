use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    }
    let mut h = HashMap::new();
    for k in &a {
        let v = h.entry(k).or_insert(0);
        *v += 1;
    }
    for k in &b {
        let v = h.entry(k).or_insert(0);
        *v -= 1;
        if *v < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
