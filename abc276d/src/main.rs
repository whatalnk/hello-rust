use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let mut g = 0;
    for ai in a.iter().take(n) {
        g = num::integer::gcd(g, *ai);
    }
    let mut ans = 0;
    for ai in a.iter_mut().take(n) {
        *ai /= g;
        while *ai % 2 == 0 {
            *ai /= 2;
            ans += 1;
        }
        while *ai % 3 == 0 {
            *ai /= 3;
            ans += 1;
        }
        if *ai != 1 {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}
