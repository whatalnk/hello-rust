use proconio::input;

fn main() {
    input! {
        a: [usize; 10]
    }
    let mut ans = 0;
    ans = a[ans];
    ans = a[ans];
    ans = a[ans];
    println!("{}", ans);
}
