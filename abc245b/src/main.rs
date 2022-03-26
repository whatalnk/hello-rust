use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }
    for i in 0..=2000 {
        if !a.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
