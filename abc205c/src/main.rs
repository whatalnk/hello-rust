use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64
    }
    if c % 2 == 0 {
        if a.abs() < b.abs() {
            println!("<");
        } else if a.abs() == b.abs() {
            println!("=");
        } else {
            println!(">");
        }
    } else {
        if a == b {
            println!("=");
        } else if a >= 0 && b >= 0 {
            if a < b {
                println!("<");
            } else {
                println!(">");
            }
        } else if a >= 0 && b < 0 {
            println!(">");
        } else if a < 0 && b >= 0 {
            println!("<");
        } else {
            if a > b {
                println!("<");
            } else {
                println!(">");
            }
        }
    }
}
