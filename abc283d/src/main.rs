use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut v = vec![0; n];
    let mut l = vec![];
    for i in 0..n {
        if s[i] == '(' {
            l.push(i);
        } else if s[i] == ')' {
            let j = l.pop().unwrap();
            v[i] = j;
        }
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        if s[i] == '(' {
            continue;
        } else if s[i] == ')' {
            let j = v[i];
            let keys = hm.keys().cloned().collect::<Vec<char>>();
            for key in &keys {
                if let Some(val) = hm.get(key) {
                    if val >= &j {
                        hm.remove(key);
                    }
                }
            }
        } else {
            if hm.contains_key(&s[i]) {
                println!("No");
                return;
            }
            hm.insert(s[i], i);
        }
    }
    println!("Yes");
}
