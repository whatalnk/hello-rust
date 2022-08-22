use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: i64,
        a: [i64; n-1],
        xy: [(usize, i64); m],
    }
    let mut b = vec![0; n];
    for i in 0..m {
        let (x, y) = xy[i];
        b[x - 1] = y;
    }
    let mut ans = true;
    for i in 0..(n - 1) {
        if t >= a[i] {
            t -= a[i];
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
