use proconio::input;

fn main() {
    input! {
        x: i64
    }
    if x >= 0 {
        println!("{}", x / 10);
    } else {
        if x % 10 == 0 {
            println!("{}", x / 10);
        } else {
            println!("{}", x / 10 - 1);
        }
    }
}
