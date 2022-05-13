use proconio::input;

fn main() {
    input! {
        mut n: i128,
    }
    let mut ans = Vec::new();
    while n > 0 {
        if n % 2 == 0 {
            n /= 2;
            ans.push("B");
        } else {
            n -= 1;
            ans.push("A");
        }
    }
    ans.reverse();
    println!("{}", ans.join(""));
}
