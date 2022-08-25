use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        r: i64,
        a: [i64; n],
    }
    println!("{} {} {} {}", n, p, q, r);
    println!("{:?}", a);
}
