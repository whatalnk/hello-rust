use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut vol = vec![false; n + 2];
    let mut sold: i64 = 0;
    for i in 0..n {
        if a[i] >= vol.len() {
            sold += 1;
        } else if vol[a[i]] {
            sold += 1;
        } else {
            vol[a[i]] = true;
        }
    }

    let mut l: usize = 1;
    let mut r: usize = n + 1;
    loop {
        while vol[l] {
            l += 1;
        }
        while r != 0 && !vol[r] {
            r -= 1;
        }
        if sold >= 2 {
            sold -= 2;
            vol[l] = true;
        } else {
            if l >= r {
                break;
            }
            vol[r] = false;
            sold += 1;
        }
    }
    println!("{}", l - 1);
}
