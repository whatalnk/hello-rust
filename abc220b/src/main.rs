use proconio::input;

fn main() {
    input! {
        n: u32,
        a: String,
        b: String

    }
    let aa = i64::from_str_radix(&a, n).unwrap();
    let bb = i64::from_str_radix(&b, n).unwrap();
    println!("{}", aa * bb);
}
