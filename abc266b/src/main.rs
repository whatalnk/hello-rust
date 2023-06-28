use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let d: i64 = 998_244_353;
    if n % d >= 0 {
        println!("{}", (n % d))
    } else {
        println!("{}", (d + (n % d)))
    }
}
