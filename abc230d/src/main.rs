use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        mut lr: [(i64, i64); n]
    }
    lr.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ans = 0;
    let mut x = -(1i64 << 40);
    for lri in lr.iter().take(n) {
        let (l, r) = lri;
        if &(x + d - 1) < l {
            ans += 1;
            x = *r;
        }
    }
    println!("{}", ans);
}
