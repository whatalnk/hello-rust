use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: Chars,
        mut w: [i64; n]
    }
    let mut hm0 = HashMap::<i64, i64>::new();
    let mut hm1 = HashMap::<i64, i64>::new();
    let mut adult = 0;
    for i in 0..n {
        let si = s[i].to_digit(10).unwrap() as i64;
        adult += si;
        let e0 = hm0.entry(w[i]).or_insert(0);
        let e1 = hm1.entry(w[i]).or_insert(0);
        if si == 0 {
            *e0 += 1;
        } else {
            *e1 += 1;
        }
    }
    let mut ww = hm0.keys().cloned().collect::<Vec<i64>>();
    ww.sort();
    let mut ans = adult;
    for i in 0..(ww.len()) {
        adult += hm0[&ww[i]];
        adult -= hm1[&ww[i]];
        ans = ans.max(adult);
    }
    println!("{}", ans);
}
