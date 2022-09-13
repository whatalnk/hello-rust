use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [String; n],
        tt: [String; m],
    }
    println!("{} {}", n, m);
    for i in 0..n {
        println!("{}", ss[i]);
    }
    for i in 0..m {
        println!("{}", tt[i]);
    }
}
