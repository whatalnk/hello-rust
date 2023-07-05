use proconio::input;

fn main() {
    input!(t: usize);
    for _ in 0..t {
        input!(n: usize);
        input!(a: [i64; n]);
        let mut ans: i64 = 0;
        for ai in &a {
            if ai % 2 != 0 {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}
