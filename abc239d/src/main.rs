use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    let mut p = vec![true; 201];
    p[0] = false;
    p[1] = false;
    for i in 2..=100 {
        if p[i] {
            let mut j = i + i;
            while j <= 200 {
                p[j] = false;
                j += i;
            }
        }
    }
    for i in a..=b {
        let mut ans = true;
        for j in c..=d {
            if p[i + j] {
                ans = false;
                break;
            }
        }
        if ans {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
