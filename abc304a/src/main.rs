use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        sa: [(String, i64); n],
    }
    if let Some(elem) = sa.iter().min_by(|x, y| x.1.cmp(&y.1)) {
        if let Some(idx) = sa.iter().position(|x| x == elem) {
            for sai in sa.iter().take(n).skip(idx) {
                println!("{}", sai.0);
            }
            for sai in sa.iter().take(idx) {
                println!("{}", sai.0);
            }
        }
    }
}
