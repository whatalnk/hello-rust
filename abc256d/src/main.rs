use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n]
    }
    let mmax = 200_000 + 1;
    let mut m = vec![0; mmax];
    for lri in lr.iter().take(n) {
        let (l, r) = lri;
        m[*l] += 1;
        m[*r] -= 1;
    }
    for i in 1..mmax {
        m[i] += m[i - 1];
    }
    let mut ans = vec![];
    let mut on = false;
    let mut left = 0;
    for (i, mi) in m.iter().enumerate().take(mmax) {
        if on && mi == &0 {
            on = false;
            ans.push((left, i));
        } else if !on && mi > &0 {
            on = true;
            left = i;
        }
    }
    for x in &ans {
        println!("{} {}", x.0, x.1);
    }
}
