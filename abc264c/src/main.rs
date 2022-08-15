use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[i64; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[i64; w2]; h2]
    }
    println!("{} {}", h1, w1);
    println!("{:?}", a);
    println!("{} {}", h2, w2);
    println!("{:?}", b);
}
