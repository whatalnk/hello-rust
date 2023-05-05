use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }
    if (a, b) == (1, 10) || (a, b) == (10, 1) || (a - b).abs() == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
