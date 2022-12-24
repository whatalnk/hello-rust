use proconio::input;

fn main() {
    input!(n: usize);
    input!(mut a: [i64; n]);
    input!(q: usize);
    for _ in 0..q {
        input!(t: usize);
        match t {
            1 => {
                input!(k: usize, x: i64);
                a[k - 1] = x;
            }
            _ => {
                input!(k: usize);
                println!("{}", a[k - 1]);
            }
        }
    }
}
