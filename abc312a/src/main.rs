use std::println;

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let chord = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];
    if chord.contains(&s.as_str()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
