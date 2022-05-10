use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; n]
    }
    println!("{} {}", n, q);
    println!("{:?}", x);
}
