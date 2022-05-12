use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }
    let l = n.len();
    let mut ans = 0;
    for i in 0..(1 << l) {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for j in 0..l {
            if (i & (j << 1)) > 0 {
                left.push(n[j])
            } else {
                right.push(n[j]);
            }
        }
        if left.len() > 0 && right.len() > 0 {
            left.sort();
            left.reverse();
            right.sort();
            right.reverse();
            let n_left: i64 = left
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse()
                .unwrap();
            let n_right: i64 = right
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse()
                .unwrap();
            ans = ans.max(n_left * n_right);
        }
    }
    println!("{}", ans);
}
