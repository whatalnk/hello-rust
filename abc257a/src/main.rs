use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    }
    let c: u8 = b'A';
    let ans = (c - 1) + (x / n) as u8;
    if x % n == 0 {
        println!("{}", ans as char);
    } else {
        println!("{}", (ans + 1) as char);
    }
}
