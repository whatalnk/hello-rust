use proconio::input;
use std::collections::HashMap;

fn main() {
    input!(q: usize);

    let mut s = HashMap::new();

    for _ in 0..q {
        input!(kind: u32);
        match kind {
            1 => {
                input!(x: i32);
                let e = s.entry(x).or_insert(0);
                *e += 1;
            }
            2 => {
                input!(x: i32, c: i32);
                if let Some(v) = s.get(&x) {
                    if &c >= v {
                        s.remove(&x);
                    } else {
                        s.entry(x).and_modify(|e| *e -= c);
                    }
                }
            }
            3 => {
                let xmin = s.keys().min().unwrap();
                let xmax = s.keys().max().unwrap();
                println!("{}", xmax - xmin);
            }
            _ => unreachable!(),
        }
    }
}
