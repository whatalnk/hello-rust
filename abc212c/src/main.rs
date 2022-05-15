use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m]
    }
    a.sort();
    b.sort();
    let mut ans: i64 = 1_000_000_000;
    let mut x = 0;
    let mut y = 0;
    while (x < n) && (y < m) {
        ans = ans.min((a[x] - b[y]).abs());
        if a[x] > b[y] {
            y += 1;
        } else {
            x += 1;
        }
    }
    println!("{}", ans);
}
