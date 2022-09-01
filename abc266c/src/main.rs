use proconio::input;

fn cross_product(p1: (i64, i64), o: (i64, i64), p2: (i64, i64)) -> i64 {
    let op1 = (p1.0 - o.0, p1.1 - o.1);
    let op2 = (p2.0 - o.0, p2.1 - o.1);
    return op1.0 * op2.1 - op1.1 * op2.0;
}

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
        d: (i64, i64),
    }
    let adc = cross_product(a, d, c);
    let dcb = cross_product(d, c, b);
    let cba = cross_product(c, b, a);
    let bad = cross_product(b, a, d);
    if adc <= 0 || dcb <= 0 || cba <= 0 || bad <= 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
