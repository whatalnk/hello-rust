use proconio::input;

const MD: i64 = 1_000_000_007;
fn powmod(x: i64, y: usize) -> i64 {
    let mut res = 1;
    for _ in 0..y {
        res = res * x % MD;
    }
    res
}

fn main() {
    input! {
        n: usize
    }
    let mut ans = powmod(10, n) - powmod(9, n) - powmod(9, n) + powmod(8, n);
    ans %= MD;
    ans = (ans + MD) % MD;
    println!("{}", ans);
}
