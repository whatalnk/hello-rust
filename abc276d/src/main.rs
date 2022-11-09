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
    let mut cnt2 = HashSet::new();
    let mut cnt3 = HashSet::new();
    let mut c2 = 0;
    let mut c3 = 0;
    for i in 0..n {
        let mut x = a[i];
        let mut cnt = 0;
        while x % 2 == 0 {
            x /= 2;
            cnt += 1;
        }
        cnt2.insert(cnt);
        c2 = c2.max(cnt);
        let mut cnt = 0;
        while x % 3 == 0 {
            x /= 3;
            cnt += 1;
        }
        cnt3.insert(cnt);
        c3 = c3.max(cnt);
        b.insert(x);
    }
    if b.len() == 1 {
        if cnt2.len() == 1 {
            println!("{}", c3)
        } else if cnt3.len() == 1 {
            println!("{}", c2)
        } else {
            println!("{}", c2 + c3);
        }
    } else {
        println!("-1");
    }
}
