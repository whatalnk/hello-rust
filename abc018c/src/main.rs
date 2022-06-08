use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        s: [Chars; r]
    }
    let mut ss: Vec<Vec<i64>> = vec![];
    for i in 0..r {
        let mut row = vec![];
        for j in 0..c {
            if s[i][j] == 'x' {
                row.push(0);
            } else {
                row.push(1);
            }
        }
        ss.push(row);
    }
    let mut rowsum = vec![];
    for i in 0..r {
        let mut row = vec![];
        let mut cnt = 0;
        for j in 0..c {
            if j < 2 * k - 1 {
                cnt += ss[i][j];
                row.push(cnt);
            } else {
                cnt -= ss[i][j - (2 * k - 1)];
                cnt += ss[i][j];
                row.push(cnt);
            }
        }
        rowsum.push(row);
    }
    let mut colsum = vec![];
    for j in 0..c {
        let mut col = vec![];
        let mut cnt = 0;
        for i in 0..r {
            if i < 2 * k - 1 {
                cnt += ss[i][j];
                col.push(cnt);
            } else {
                cnt -= ss[i - (2 * k - 1)][j];
                cnt += ss[i][j];
                col.push(cnt);
            }
        }
        colsum.push(col);
    }
    let mut ans = 0;
    for x in (k - 1)..=(r - k) {
        for y in (k - 1)..=(c - k) {
            if rowsum[x][y] + colsum[y][x] == 4 * k as i64 - 3 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
