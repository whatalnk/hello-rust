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
    for i in b'a'..=b'z' {
        y.push(i as char);
    }
    let mut xy = HashMap::new();
    let mut yx = HashMap::new();
    for i in 0..l {
        xy.insert(x[i], y[i]);
        yx.insert(y[i], x[i]);
    }
    let mut sy = Vec::new();
    for si in s.iter().take(n) {
        let ss: String = si
            .iter()
            .map(|c| xy.get(&c).unwrap())
            .fold("".to_string(), |acc, c| acc + &c.to_string());
        sy.push(ss);
    }
    sy.sort();
    for syi in sy.iter().take(n) {
        let ss: String = syi
            .as_str()
            .chars()
            .map(|c| yx.get(&c).unwrap())
            .fold("".to_string(), |acc, c| acc + &c.to_string());
        println!("{}", ss);
    }
}
