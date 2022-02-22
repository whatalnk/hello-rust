use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
        a: i32
    }
    if (a + k - 1) % n == 0 {
        println!("{}", n);
    } else {
        println!("{}", (a + k - 1) % n);
    }
}
