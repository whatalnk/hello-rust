use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", n);
    println!("{:?}", a);
}
