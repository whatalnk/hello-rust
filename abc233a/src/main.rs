use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    let mut cnt = 0;
    while x + cnt * 10 < y {
        cnt += 1;
    }
    println!("{}", cnt);
}
