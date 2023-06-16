use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }
    let mut x = h[0];
    for hi in h.iter().take(n).skip(1) {
        if hi > &x {
            x = *hi;
        } else {
            println!("{}", x);
            return;
        }
    }
    println!("{}", x);
}
