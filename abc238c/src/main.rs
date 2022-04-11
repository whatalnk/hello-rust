use proconio::input;

fn f(mut x: i64) -> i64 {
    let mut ans = 0;
    while x > 0 {
        x /= 10;
        ans += 1;
    }
    return ans;
}

fn main() {
    input! {
        n: i64
    }
    let ans = f(n);
    println!("{} {}", n, ans);
}
