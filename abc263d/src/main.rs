use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [i64; n],
    }
    println!("{} {} {}", n, l, r);
    println!("{:?}", a);
}
