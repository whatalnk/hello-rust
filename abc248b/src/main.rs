use proconio::input;

fn main() {
    input! {
        mut a: i128,
        b: i128,
        k: i128
    }

    if a >= b {
        println!("0");
        return;
    }
    let mut ans = 0;
    while a < b {
        a *= k;
        ans += 1;
    }
    println!("{}", ans);
}
