use proconio::input;

fn main() {
    input! {
        mut k: i64,
    }
    let mut p: i64 = 2;
    let mut ans: i64 = 1;
    while p * p <= k {
        let mut e: i64 = 0;
        while k % p == 0 {
            k /= p;
            e += 1;
        }
        let mut n = 0;
        while e > 0 {
            n += p;
            let mut x = n;
            while x % p == 0 {
                x /= p;
                e -= 1;
            }
        }
        ans = ans.max(n);
        p += 1;
    }
    ans = ans.max(k);
    println!("{}", ans);
}
