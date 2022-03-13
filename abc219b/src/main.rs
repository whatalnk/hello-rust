use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: Chars
    }
    let mut ans = String::new();
    for i in &t {
        match i {
            '1' => ans += &s1,
            '2' => ans += &s2,
            '3' => ans += &s3,
            _ => break,
        }
    }
    println!("{}", ans);
}
