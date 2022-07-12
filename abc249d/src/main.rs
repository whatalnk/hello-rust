use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let n_max = 200000;
    let mut cnt = vec![0; n_max + 1];
    for i in 0..n {
        cnt[a[i] as usize] += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        let mut j = 1;
        while j * j <= a[i] {
            if a[i] % j == 0 {
                let cnt_j = cnt[j as usize];
                let cnt_k = cnt[(a[i] / j) as usize];
                ans += cnt_j * cnt_k * 2;
            }
            j += 1;
        }
    }
    println!("{}", ans);
}
