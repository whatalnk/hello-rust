use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i128,
        mut ab: [(i128, i128); n]
    }
    ab.sort();
    let mut m = k;
    let mut cur = 0;
    for (a, b) in ab {
        let r = a - cur;
        if m >= r {
            m -= r;
            m += b;
            cur = a;
        } else {
            break;
        }
    }
    println!("{}", cur + m);
}
