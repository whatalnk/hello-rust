use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize
    }
    let mut hm: HashMap<i64, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let e = hm.entry(a[i]).or_insert(vec![]);
        e.push(i + 1);
    }
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: i64
        }
        // println!("{} {} {}", l, r, x);
        if hm.contains_key(&x) {
            let aa = hm.get(&x).unwrap();
            if r < *aa.first().unwrap() || l > *aa.last().unwrap() {
                println!("0");
            } else {
                // println!("{:?}", aa);
                let li = match aa.binary_search(&l) {
                    Ok(v) => v,
                    Err(v) => v,
                };
                let ri = match aa.binary_search(&r) {
                    Ok(v) => v,
                    Err(v) => v,
                };
                // println!("{} {}", li, ri);
                println!("{}", ri - li);
            }
        } else {
            println!("0");
        }
    }
}
