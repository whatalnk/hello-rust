use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [i128; n]
    }
    let m: i128 = 1_000_000_000 + 7;
    let mut ans: i128 = c[0];
    ans %= m;
    for i in 1..n {
        ans *= c[i] - i as i128;
        ans %= m;
    }
    println!("{}", ans);
}
