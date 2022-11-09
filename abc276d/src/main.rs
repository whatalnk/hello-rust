use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let mut g = 0;
    for i in 0..n {
        g = num::integer::gcd(g, a[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        a[i] /= g;
        while a[i] % 2 == 0 {
            a[i] /= 2;
            ans += 1;
        }
        while a[i] % 3 == 0 {
            a[i] /= 3;
            ans += 1;
        }
        if a[i] != 1 {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}
