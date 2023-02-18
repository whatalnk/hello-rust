use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    };
    s.reverse();
    for ss in s.iter() {
        println!("{}", ss);
    }
}
