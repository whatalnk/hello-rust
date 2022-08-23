use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: i64,
        a: [i64; n-1],
        xy: [(usize, i64); m],
    }
    let mut b = vec![0; n + 1];
    for i in 0..m {
        let (x, y) = xy[i];
        b[x] = y;
    }
    let mut ans = true;
    for i in 1..=(n - 1) {
        if t > a[i - 1] {
            t -= a[i - 1];
            t += b[i + 1];
        } else {
            ans = false;
            break;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
