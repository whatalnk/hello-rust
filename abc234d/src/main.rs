use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        p: [i64; n]
    }
    println!("{}", n);
    println!("{:?}", p);
}
