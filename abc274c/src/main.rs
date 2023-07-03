use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut p = vec![0; 2 * (n + 1)];
    let mut d = vec![0; 2 * (n + 1)];
    for i in 0..n {
        let l = 2 * (i + 1);
        let r = 2 * (i + 1) + 1;
        p[l] = a[i];
        p[r] = a[i];
        d[l] = d[a[i]] + 1;
        d[r] = d[a[i]] + 1;
    }
    for dk in d.iter().take(2 * n + 1).skip(1) {
        println!("{}", dk);
    }
}
