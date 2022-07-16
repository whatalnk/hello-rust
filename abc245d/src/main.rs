use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n + 1],
        mut c: [i64; n + m + 1]
    }
    let mut b = vec![0; m + 1];
    for i in 0..=m {
        let ii = m - i;
        b[ii] = c[ii + n] / a[n];
        for j in 0..=n {
            c[ii + j] -= b[ii] * a[j];
        }
    }
    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
