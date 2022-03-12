use proconio::input;

fn main() {
    input! {
        p: [usize; 26]
    }
    let a: Vec<&str> = "abcdefghijklmnopqrstuvwxyz".split("").collect();
    let s: String = p.iter().map(|i| a[*i]).collect();
    println!("{}", s);
}
