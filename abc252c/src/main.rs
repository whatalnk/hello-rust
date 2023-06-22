use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut m = vec![HashMap::new(); 10];
    for si in s.iter().take(n) {
        for (j, sij) in si.iter().enumerate().take(10) {
            let jj = sij.to_digit(10).unwrap() as usize;
            let e = m[jj].entry(j).or_insert(0);
            *e += 1;
        }
    }
    let mut ans = vec![];
    for mi in m.iter().take(10) {
        let mut ans_ = 0;
        for (k, v) in mi {
            ans_ = ans_.max(*k as i32 + (v - 1) * 10);
        }
        ans.push(ans_);
    }
    println!("{}", ans.iter().min().unwrap());
}
