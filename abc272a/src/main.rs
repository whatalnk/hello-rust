use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = 0i64;
    for ai in a.iter().take(n) {
        ans += ai;
    }
    println!("{}", ans);
}
