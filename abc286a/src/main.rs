use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        mut a: [String; n]
    }

    for (i, j) in (p..=q).zip(r..=s) {
        a.swap(i - 1, j - 1);
    }
    println!("{}", a.join(" "));
}
