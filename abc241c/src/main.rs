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
    let mut ans = false;
    // よこ
    for i in 0..n {
        for j in 0..n {
            if j + 5 <= n - 1 {
                let mut rs = 0;
                for d in 0..6 {
                    rs += s[i][j + d];
                }
                if rs >= 4 {
                    ans = true;
                }
            }
        }
    }
    // たて
    for j in 0..n {
        for i in 0..n {
            if i + 5 <= n - 1 {
                let mut cs = 0;
                for d in 0..6 {
                    cs += s[i + d][j];
                }
                if cs >= 4 {
                    ans = true;
                }
            }
        }
    }
    // ななめ1
    for i in 0..n {
        for j in 0..n {
            if i + 5 <= n - 1 && j + 5 <= n - 1 {
                let mut ds = 0;
                for d in 0..6 {
                    ds += s[i + d][j + d];
                }
                if ds >= 4 {
                    ans = true;
                }
            }
        }
    }
    // ななめ2
    for i in 0..n {
        for j in 0..n {
            if i >= 5 && j >= 5 {
                let mut ds = 0;
                for d in 0..6 {
                    ds += s[i - d][j - d];
                }
                if ds >= 4 {
                    ans = true;
                }
            }
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
