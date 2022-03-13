use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: Chars
    }
    let mut ans: Vec<&str> = Vec::new();
    for i in &t {
        match i {
            '1' => ans.push(&s1),
            '2' => ans.push(&s2),
            '3' => ans.push(&s3),
            _ => break,
        }
    }
    let a: String = ans.iter().map(|x| *x).collect();
    println!("{}", a);
}
