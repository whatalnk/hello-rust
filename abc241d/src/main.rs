use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input!(n: usize);
    let mut a = BinaryHeap::new();
    for _ in 0..n {
        input!(c: u32);
        match c {
            1 => {
                input!(x: i64);
                a.push(x);
            }
            2 => {
                input!(x: i64, k: usize);
                let b = a.clone().into_sorted_vec();
                let idx = b.binary_search(&x);
                match idx {
                    Ok(idx) => {
                        if k <= idx {
                            println!("{}", b[idx - k + 1]);
                        } else {
                            println!("-1");
                        }
                    }
                    Err(idx) => {
                        if k <= idx {
                            println!("{}", b[idx - k]);
                        } else {
                            println!("-1");
                        }
                    }
                }
            }
            _ => {
                input!(x: i64, k: usize);
                let b = a.clone().into_sorted_vec();
                let idx = b.binary_search(&x);
                match idx {
                    Ok(idx) => {
                        if k <= b.len() - idx {
                            println!("{}", b[idx + k - 1]);
                        } else {
                            println!("-1");
                        }
                    }
                    Err(idx) => {
                        if k <= b.len() - idx {
                            println!("{}", b[idx + k - 1]);
                        } else {
                            println!("-1");
                        }
                    }
                }
            }
        }
    }
}
