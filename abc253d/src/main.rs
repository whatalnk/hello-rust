use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64
    }
    let na = n / a;
    let nb = n / b;
    let nab = n / (a * b);
    let s = (1 + n) * n / 2;
    let sa = a * (1 + na) * na / 2;
    let sb = b * (1 + nb) * nb / 2;
    let sab = a * b * (1 + nab) * nab / 2;
    println!("{}", s - (sa + sb - sab));
}
