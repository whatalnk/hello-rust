use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                if a[i][j] != '-' {
                    println!("incorrect");
                    return;
                } else {
                    continue;
                }
            } else if (a[i][j] == 'D' && a[j][i] == 'D')
                || (a[i][j] == 'W' && a[j][i] == 'L')
                || (a[i][j] == 'L' && a[j][i] == 'W')
            {
                continue;
            } else {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
}
