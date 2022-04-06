use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    }
    let mut s = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == '#' {
                s[i][j] = 1;
            }
        }
    }
    // よこ
    for i in 0..n {
        for j in 0..n {
            if j + 5 > n - 1 {
                break;
            }
            let mut rs = 0;
            for d in 0..6 {
                rs += s[i][j + d];
            }
            if rs >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    // たて
    for j in 0..n {
        for i in 0..n {
            if i + 5 > n - 1 {
                break;
            }
            let mut cs = 0;
            for d in 0..6 {
                cs += s[i + d][j];
            }
            if cs >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    // ななめ1
    for i in 0..n {
        if i + 5 > n - 1 {
            break;
        }
        for j in 0..n {
            if j + 5 > n - 1 {
                break;
            }
            let mut ds = 0;
            for d in 0..6 {
                ds += s[i + d][j + d];
            }
            if ds >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    // ななめ2
    for i in 5..n {
        for j in 5..n {
            let mut ds = 0;
            for d in 0..6 {
                ds += s[i - d][j - d];
            }
            if ds >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
