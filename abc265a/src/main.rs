use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        n: i64,
    }
    let z = y.min(x * 3);
    let ans = (n / 3) * z + (n % 3) * x;
    println!("{}", ans);
}
