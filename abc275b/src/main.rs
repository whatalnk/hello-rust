use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
    }
    let m = 998_244_353;
    let mut abc = a % m;
    abc *= b % m;
    abc %= m;
    abc *= c % m;
    abc %= m;
    let mut def = d % m;
    def *= e % m;
    def %= m;
    def *= f % m;
    def %= m;
    let mut ans = abc - def;
    ans += m;
    ans %= m;
    println!("{}", ans);
}
