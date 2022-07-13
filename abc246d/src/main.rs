use proconio::input;

fn f(a: i64, b: i64) -> i64 {
    return a * a * a + a * a * b + a * b * b + b * b * b;
}

fn main() {
    input! {
        n: i64
    }
    let a_max = 1000000;
    let mut ans: i64 = 1_000_000_000_000_000_000;
    for i in 0..=a_max {
        let mut left = -1;
        let mut right = a_max;
        let mut mid = (left + right) / 2;
        while left + 1 != right {
            if f(i, mid) >= n {
                right = mid;
            } else {
                left = mid;
            }
            mid = (left + right) / 2;
        }
        ans = ans.min(f(i, right));
    }
    println!("{}", ans);
}
