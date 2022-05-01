use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut ans = 0;
    for i in 0..n {
        let c = a[(i + 1)..].iter().filter(|&x| *x == a[i]).count();
        ans += n - i - 1;
        ans -= c;
    }
    println!("{}", ans);
}
