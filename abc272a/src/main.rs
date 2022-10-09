use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = 0i64;
    for i in 0..n {
        ans += a[i];
    }
    println!("{}", ans);
}
