use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    if s.len() != 8 {
        println!("No");
        return;
    }
    if !s[0].is_ascii_uppercase() {
        println!("No");
        return;
    }
    if !s[7].is_ascii_uppercase() {
        println!("No");
        return;
    }
    for i in 1..=6 {
        if !s[i].is_ascii_digit() {
            println!("No");
            return;
        }
    }
    if s[1] == '0' {
        println!("No");
        return;
    }
    println!("Yes");
}
