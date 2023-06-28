use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        r: i64,
        a: [i64; n],
    }
    let mut b: Vec<i64> = vec![0; n + 1];
    b[0] = 0;
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }
    let mut hs = HashSet::<i64>::new();
    for bi in b.iter().take(n + 1) {
        hs.insert(*bi);
    }
    for bx in b.iter().take(n + 1) {
        if hs.contains(&(bx + p)) && hs.contains(&(bx + p + q)) && hs.contains(&(bx + p + q + r)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
