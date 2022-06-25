use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [usize; q]
    }
    let mut m = vec![false; n];
    for i in 0..k {
        m[a[i] - 1] = true;
    }
    for i in 0..q {
        if a[l[i] - 1] - 1 < n - 1 && !m[a[l[i] - 1]] {
            m[a[l[i] - 1] - 1] = false;
            a[l[i] - 1] += 1;
            m[a[l[i] - 1] - 1] = true;
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
