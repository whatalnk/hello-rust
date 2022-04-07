use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n]
    }
    let mut m = vec![vec![false; x + 1]; n + 1];
    m[0][0] = true;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..(x + 1) {
            if m[i][j] {
                if j + a <= x {
                    m[i + 1][j + a] = true;
                }
                if j + b <= x {
                    m[i + 1][j + b] = true;
                }
            }
        }
    }
    if m[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
