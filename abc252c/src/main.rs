use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut m = vec![HashMap::new(); 10];
    for i in 0..n {
        for j in 0..10 {
            let jj = s[i][j].to_digit(10).unwrap() as usize;
            let e = m[jj].entry(j).or_insert(0);
            *e += 1;
        }
    }
    let mut ans = vec![];
    for i in 0..10 {
        let mut ans_ = 0;
        for (k, v) in &m[i] {
            ans_ = ans_.max(k.clone() as i32 + (v - 1) * 10);
        }
        ans.push(ans_);
    }
    println!("{}", ans.iter().min().unwrap());
}
