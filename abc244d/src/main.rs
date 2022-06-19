use proconio::input;

fn main() {
    input! {
        mut s: [char; 3],
        mut t: [char; 3]
    }
    let mut cnt = 0;
    for i in 0..3 {
        if s[i] != t[i] {
            cnt += 1;
        }
    }
    if cnt == 2 {
        println!("No");
    } else {
        println!("Yes");
    }
}
