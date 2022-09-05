use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    if s[0] != '0' {
        println!("No");
        return;
    }
    let mut b = vec![0; 7];
    let m = vec![3, 2, 4, 1, 3, 5, 0, 2, 4, 6];
    for i in 0..10 {
        if s[i] == '1' {
            b[m[i]] += 1;
        }
    }

    for i in 0..7 {
        for j in (i + 2)..7 {
            for k in (i + 1)..j {
                if b[i] > 0 && b[j] > 0 && b[k] == 0 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
