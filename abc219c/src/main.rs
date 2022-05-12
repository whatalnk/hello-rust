use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n]
    }
    let l = x.len();
    let mut y = Vec::new();
    for i in ('a' as u8)..=('z' as u8) {
        y.push(i as char);
    }
    let mut xy = HashMap::new();
    let mut yx = HashMap::new();
    for i in 0..l {
        xy.insert(x[i], y[i]);
        yx.insert(y[i], x[i]);
    }
    let mut sy = Vec::new();
    for i in 0..n {
        let ss: String = s[i]
            .iter()
            .map(|c| xy.get(&c).unwrap())
            .fold("".to_string(), |acc, c| acc + &c.to_string());
        sy.push(ss);
    }
    sy.sort();
    for i in 0..n {
        let ss: String = sy[i]
            .as_str()
            .chars()
            .map(|c| yx.get(&c).unwrap())
            .fold("".to_string(), |acc, c| acc + &c.to_string());
        println!("{}", ss);
    }
}
