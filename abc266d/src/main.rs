use proconio::input;

fn main() {
    input! {
        n: usize,
        txa: [(i64, i64, i64); n],
    }
    println!("{}", n);
    for x in &txa {
        println!("{:?}", x);
    }
}
