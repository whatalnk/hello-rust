use proconio::input;

fn main() {
    input! {
        y: i64,
    }
    let r = y % 4;
    if r < 2 {
        println!("{}", y + 2 - r);
    } else if r == 2 {
        println!("{}", y);
    } else {
        println!("{}", y + 3);
    }
}
