use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input!(n: usize);
    input!(mut a: [i64; n]);
    input!(q: usize);
    let mut is_replaced = false;
    let mut replaced_to = 0;
    let mut m = BTreeMap::<usize, i64>::new();
    for _ in 0..q {
        input!(c: usize);
        match c {
            1 => {
                input!(x: i64);
                is_replaced = true;
                replaced_to = x;
                m.clear();
            }
            2 => {
                input!(i: usize);
                input!(x: i64);
                if is_replaced {
                    let e = m.entry(i).or_insert(0);
                    *e += x;
                } else {
                    a[i - 1] += x;
                }
            }
            _ => {
                input!(i: usize);
                if is_replaced {
                    if let Some(v) = m.get(&i) {
                        println!("{}", replaced_to + v);
                    } else {
                        println!("{}", replaced_to);
                    }
                } else {
                    println!("{}", a[i - 1]);
                }
            }
        }
    }
}
