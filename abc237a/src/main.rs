use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    if n >= -2_147_483_648 && n < 2_147_483_648 {
        println!("Yes");
    } else {
        println!("No");
    }
}
