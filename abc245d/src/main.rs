use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n + 1],
        c: [i64; n + m + 1]
    }
    let mut b = vec![0; m + 1];
    b[m] = c[m + n] / a[n];
    for i in 0..(n + m + 1) {
        let ii = n + m - i;
        let mut cc = c[ii];
        for j in 0..m {
            if ii >= j && ii <= m + j {
                cc -= a[j] * b[ii - j];
            }
        }
        if i > 0 && i <= m {
            b[m - i] = cc / a[n];
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
