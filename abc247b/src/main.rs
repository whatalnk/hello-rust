use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n]
    }
    for i in 0..n {
        let (s1, t1) = &st[i];
        let mut ans1 = true;
        let mut ans2 = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            let (s2, t2) = &st[j];
            if s1 == s2 || s1 == t2 {
                ans1 = false;
            }
            if t1 == s2 || t1 == t2 {
                ans2 = false;
            }
        }
        if !ans1 && !ans2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
