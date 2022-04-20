use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[i64; m]; n]
    }
    let mut ans = true;
    for i in 0..(n - 1) {
        for j in 0..m {
            if b[i + 1][j] - b[i][j] != 7 {
                ans = false;
                break;
            }
        }
    }
    for i in 0..n {
        for j in 0..(m - 1) {
            if b[i][j + 1] - b[i][j] != 1 {
                ans = false;
                break;
            }
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
