use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    println!("{} {}", h, w);
    for i in 0..h {
        println!("{:?}", c[i]);
    }
}
