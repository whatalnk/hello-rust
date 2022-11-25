use proconio::input;

fn main() {
    input!(n: usize);
    input!(mut a: [i64; n]);
    input!(q: usize);
    for _ in 0..q {
        input!(c: usize);
        match c {
            1 => {
                input!(x: i64);
                a = vec![x; n];
            }
            2 => {
                input!(i: usize);
                input!(x: i64);
                a[i - 1] += x;
            }
            _ => {
                input!(i: usize);
                println!("{}", a[i - 1]);
            }
        }
    }
}
