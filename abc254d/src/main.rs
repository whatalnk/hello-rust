use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut sq = vec![false; n + 1];
    let mut i = 1;
    while i * i <= n {
        sq[i * i] = true;
        i += 1;
    }
    let mut d = vec![Vec::<usize>::new(); n + 1];
    for i in 1..=n {
        let mut j = i;
        while j <= n {
            d[j].push(i);
            j += i;
        }
    }
    let mut cnt = vec![0; n + 1];
    for i in 1..=n {
        let mut f = 0;
        for j in 0..d[i].len() {
            if sq[d[i][j]] {
                f = d[i][j];
            }
        }
        cnt[i / f] += 1;
    }
    let mut ans = 0;
    for cnti in cnt.iter().take(n + 1).skip(1) {
        ans += cnti * cnti;
    }
    println!("{}", ans);
}
