use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [i128; n]
    }
    c.sort();
    let m: i128 = 1_000_000_000 + 7;
    let mut ans: i128 = 1;
    for i in 0..n {
        ans *= c[i] - i as i128;
        ans %= m;
    }
    println!("{}", ans);
}
