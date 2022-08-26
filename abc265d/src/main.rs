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
    for i in 0..=n {
        hs.insert(b[i]);
    }
    for x in 0..=n {
        if hs.contains(&(b[x] + p))
            && hs.contains(&(b[x] + p + q))
            && hs.contains(&(b[x] + p + q + r))
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
