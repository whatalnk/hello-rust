use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!("{}", n);
    println!("{:?}", a);
}
