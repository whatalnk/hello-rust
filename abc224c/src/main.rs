use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i128, i128); n]
    }
    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if i != j && j != k && i != k {
                    // println!("{} {} {}", i, j, k);
                    let a = xy[i];
                    let b = xy[j];
                    let c = xy[k];
                    let ab = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2);
                    let bc = (b.0 - c.0).pow(2) + (b.1 - c.1).pow(2);
                    let ca = (c.0 - a.0).pow(2) + (c.1 - a.1).pow(2);
                    if ab == bc + ca || bc == ca + ab || ca == ab + bc {
                        continue;
                    } else {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
