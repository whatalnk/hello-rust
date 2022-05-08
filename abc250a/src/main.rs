use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
        r: i32,
        c: i32
    }
    let mut ans = 0;
    for i in 1..=h {
        for j in 1..=w {
            if (i - r).abs() + (j - c).abs() == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
