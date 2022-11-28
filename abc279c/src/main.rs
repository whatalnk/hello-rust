use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }
    println!("{:?}", s);
    println!("{:?}", t);
}
