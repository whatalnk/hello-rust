use proconio::input;

fn main() {
    input! {
        mut n: i128
    }
    let mut b = vec![];
    while n > 0 {
        if n % 2 == 0 {
            b.push('0');
        } else {
            b.push('2');
        }
        n /= 2;
    }
    let ans: String = b.iter().rev().collect();
    println!("{}", ans);
}
