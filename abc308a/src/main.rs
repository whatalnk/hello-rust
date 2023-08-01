use std::println;

use proconio::input;

fn main() {
    input! {
        a: [i64; 8],
    }
    for c in a.windows(2) {
        if c[0] > c[1]
            || c[0] < 100
            || c[0] > 675
            || c[1] < 100
            || c[1] > 675
            || c[0] % 25 != 0
            || c[1] % 25 != 0
        {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
