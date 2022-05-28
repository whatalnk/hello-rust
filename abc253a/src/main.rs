use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    }
    if b >= a && b <= c {
        println!("Yes");
    } else if b >= c && b <= a {
        println!("Yes");
    } else {
        println!("No");
    }
}
