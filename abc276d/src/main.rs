use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    if a.iter().min() == a.iter().max() {
        println!("0");
        return;
    }
    let mut b = HashSet::new();
    let mut cnt2 = 0;
    let mut cnt3 = 0;
    for i in 0..n {
        let mut x = a[i];
        let mut cnt = 0;
        while x % 2 == 0 {
            x /= 2;
            cnt += 1;
        }
        cnt2 = cnt2.max(cnt);
        let mut cnt = 0;
        while x % 3 == 0 {
            x /= 3;
            cnt += 1;
        }
        cnt3 = cnt3.max(cnt);
        b.insert(x);
    }
    if b.len() == 1 {
        println!("{}", cnt2 + cnt3);
    } else {
        println!("-1");
    }
}
