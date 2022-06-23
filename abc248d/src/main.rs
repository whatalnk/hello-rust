use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize
    }
    let mut v = vec![];
    v.push(HashMap::<i64, i64>::new());
    for i in 0..n {
        if i == 0 {
            let mut hm = HashMap::new();
            hm.insert(a[i], 1i64);
            v.push(hm);
        } else {
            let mut hm = v[i].clone();
            let e = hm.entry(a[i]).or_insert(0);
            *e += 1;
            v.push(hm);
        }
    }
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: i64
        }
        let mut right = 0;
        if let Some(i) = v[r].get(&x) {
            right = *i;
        }
        let mut left = 0;
        if let Some(i) = v[l - 1].get(&x) {
            left = *i;
        }
        println!("{}", right - left);
    }
}
