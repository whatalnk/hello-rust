use proconio::input;

fn main() {
    input! {
        mut x: i64,
        k: usize,
    }
    let mut b = 1;
    for _ in 0..k {
        let r = (x / b) % 10;
        b *= 10;
        x /= b;
        if r >= 5 {
            x += 1;
        }
        x *= b;
    }
    println!("{}", x);
}
