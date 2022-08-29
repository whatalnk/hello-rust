use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let d = 998244353i64;
    if n % d >= 0 {
        println!("{}", (n % d))
    } else {
        println!("{}", (d + (n % d)))
    }
}
