use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        abc: Chars,

    }
    let a = abc[0].to_digit(10).unwrap();
    let b = abc[1].to_digit(10).unwrap();
    let c = abc[2].to_digit(10).unwrap();
    println!("{}", (a + b + c) * 111);
}
