use proconio::input;

fn main() {
    input! {
        n: usize,
        xyp: [(isize, isize, i64); n]
    }
    println!("{}", n);
    for i in 0..n {
        println!("{:?}", xyp[i]);
    }
}
