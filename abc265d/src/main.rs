use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        r: i64,
        a: [i64; n],
    }
    let pqr = p + q + r;
    let mut b = vec![0; n];
    b[0] = a[0];
    for i in 1..n {
        b[i] = b[i - 1] + a[i];
    }
    let mut l = 0;
    let mut r = 2;
    let mut lr = vec![];
    loop {
        if l == n - 3 {
            break;
        }
        if b[r] - b[l] == pqr {
            lr.push((l, r));
            l += 1;
        } else if b[r] - b[l] < pqr && r < n - 1 {
            r += 1;
        } else if b[r] - b[l] < pqr {
            l += 1;
        } else if b[r] - b[l] > pqr {
            l += 1;
        }
    }
    for i in 0..lr.len() {
        let (x, w) = lr[i];
        for y in (x + 1)..(w - 1) {
            if b[y] - b[x] == p {
                for z in (y + 1)..w {
                    if b[z] - b[y] == q {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
