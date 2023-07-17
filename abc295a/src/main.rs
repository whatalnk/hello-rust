use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }
    let words = vec!["and", "not", "that", "the", "you"];
    for x in &w {
        if words.contains(&x.as_str()) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
