use proconio::input;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

fn main() {
    input!(n: usize);
    let mut a = BTreeMap::new();
    let x_max: i64 = 1_000_000_000_000_000_000;
    for _ in 0..n {
        input!(c: u32);
        match c {
            1 => {
                input!(x: i64);
                let e = a.entry(x).or_insert(0);
                *e += 1
            }
            2 => {
                input!(x: i64, k: usize);
                let mut it = a.range((Included(&0), Included(&x)));
                let mut cnt = 0;
                for _ in 0..5 {
                    if let Some(e) = it.next_back() {
                        let (key, value) = e;
                        cnt += value;
                        if cnt >= k {
                            println!("{}", key);
                            break;
                        }
                    }
                }
                if cnt < k {
                    println!("-1");
                }
            }
            _ => {
                input!(x: i64, k: usize);
                let mut it = a.range((Included(x), Included(x_max)));
                let mut cnt = 0;
                for _ in 0..5 {
                    if let Some(e) = it.next() {
                        let (key, value) = e;
                        cnt += value;
                        if cnt >= k {
                            println!("{}", key);
                            break;
                        }
                    }
                }
                if cnt < k {
                    println!("-1");
                }
            }
        }
    }
}
