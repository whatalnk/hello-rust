use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
         n: usize,
         w: i64,
         a: [i64; n]
    }
    let mut hs = HashSet::new();
    for i in 0..n {
        if a[i] <= w {
            hs.insert(a[i]);
        }
        for j in (i + 1)..n {
            if a[i] + a[j] <= w {
                hs.insert(a[i] + a[j]);
            }
            for k in (j + 1)..n {
                if a[i] + a[j] + a[k] <= w {
                    hs.insert(a[i] + a[j] + a[k]);
                }
            }
        }
    }
    println!("{}", hs.len());
}
