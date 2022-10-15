use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut f = vec![0; n + 1];
    f[0] = 1;
    for i in 1..=n {
        f[i] = i * f[i - 1];
    }
    println!("{}", f[n]);
}
