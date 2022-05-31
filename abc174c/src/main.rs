use proconio::input;

fn main() {
    input! {
        k: i64
    }
    let mut x = 7 % k;
    for i in 1..=k {
        if x == 0 {
            println!("{}", i);
            return;
        }
        x = (x * 10 + 7) % k;
    }
    println!("-1");
}
