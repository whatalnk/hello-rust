use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let ss: Vec<u32> = s.iter().map(|x| x.to_digit(10).unwrap()).collect();
    for i in 0..=9 {
        if !ss.contains(&(i as u32)) {
            println!("{}", i);
        }
    }
}
