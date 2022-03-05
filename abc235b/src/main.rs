use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }
    let mut x = h[0];
    for i in 1..n {
        if h[i] > x {
            x = h[i];
        } else {
            println!("{}", x);
            return;
        }
    }
    println!("{}", x);
}
