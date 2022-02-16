use proconio::input;

fn f(x: i32) -> i32 {
    x * x + 2 * x + 3
}
fn main() {
    input! {
        t: i32,
    }
    let ans = f(f(f(t) + t) + f(f(t)));
    println!("{}", ans);
}
