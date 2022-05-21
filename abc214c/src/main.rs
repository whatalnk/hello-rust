use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
        mut t: [i64; n]
    }
    for i in 0..(n * 2) {
        t[(i + 1) % n] = t[(i + 1) % n].min(t[i % n] + s[i % n]);
    }
    for x in &t {
        println!("{}", x);
    }
}
