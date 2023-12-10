use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [String; n],
    }
    let ans = &mut s[..k];
    ans.sort();
    for x in ans.iter() {
        println!("{}", x);
    }
}
