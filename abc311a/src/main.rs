use std::println;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let mut a = false;
    let mut b = false;
    let mut c = false;
    for (i, e) in s.iter().enumerate() {
        match e {
            'A' => a = true,
            'B' => b = true,
            'C' => c = true,
            _ => continue,
        }
        if a && b && c {
            println!("{}", i + 1);
            return;
        }
    }
}
