use proconio::input;

fn main() {
    input! {
        s: i32,
        t: i32,
        x: i32
    }
    if x >= s || x < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
