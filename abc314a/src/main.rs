use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let pi100 = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
    println!("{}", pi100.chars().take(2 + n).collect::<String>());
}
