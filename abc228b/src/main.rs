use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }
    let mut b: Vec<bool> = vec![false; n];
    let mut cur = x - 1;
    b[cur] = true;
    let mut ans = 1;
    loop {
        if !b[a[cur] - 1] {
            b[a[cur] - 1] = true;
            cur = a[cur] - 1;
            ans += 1;
        } else {
            println!("{}", ans);
            return;
        }
    }
}
