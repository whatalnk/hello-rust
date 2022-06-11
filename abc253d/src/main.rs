use num::integer::lcm;
use proconio::input;

fn main() {
    input! {
        n: i128,
        a: i128,
        b: i128
    }
    let ab = lcm(a, b);
    let na = n / a;
    let nb = n / b;
    let nab = n / ab;
    let s = (1 + n) * n / 2;
    let sa = a * (1 + na) * na / 2;
    let sb = b * (1 + nb) * nb / 2;
    let sab = ab * (1 + nab) * nab / 2;
    println!("{}", s - (sa + sb - sab));
}
