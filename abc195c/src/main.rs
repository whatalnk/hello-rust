use proconio::input;

fn main() {
    input! {
        n: i128
    }
    let mut ans = 0;
    let mut st: i128 = 1;
    let mut ed: i128 = st * 1000 - 1;
    for i in 0..=5 {
        if n >= st {
            ans += i * (n.min(ed) - st + 1);
        } else {
            break;
        }
        st *= 1000;
        ed = st * 1000 - 1;
    }
    println!("{}", ans);
}
