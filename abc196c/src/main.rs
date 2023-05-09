use proconio::input;

fn f(mut n: i128) -> u32 {
    let mut ans = 0;
    while n > 0 {
        ans += 1;
        n /= 10;
    }
    ans
}

fn main() {
    input! {
        n: i128,
    }
    let l = f(n) / 2;
    let mut ans = 0;
    for i in 1..((10i128).pow(l)) {
        if i + i * 10i128.pow(f(i)) > n {
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}
