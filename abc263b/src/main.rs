use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n-1]
    }
    let mut pp = vec![0; n + 1];
    for (i, pi) in p.iter().enumerate().take(n - 1) {
        pp[i + 2] = *pi;
    }
    let mut cur = n;
    let mut ans = 0;
    while cur > 1 {
        cur = pp[cur];
        ans += 1;
    }
    println!("{}", ans);
}
