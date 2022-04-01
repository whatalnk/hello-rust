use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
        s: Chars
    }
    let mut r = HashMap::new();
    let mut l = HashMap::new();
    for i in 0..n {
        let (mut x, y) = xy[i];
        if s[i] == 'R' {
            let e = r.entry(y).or_insert(x);
            *e = *e.min(&mut x);
        } else {
            let e = l.entry(y).or_insert(x);
            *e = *e.max(&mut x);
        }
    }
    let mut ans = false;
    for k in l.keys() {
        if r.contains_key(&k) {
            let lv = l.get(k).unwrap();
            let rv = r.get(k).unwrap();
            if rv <= lv {
                ans = true;
                break;
            }
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
