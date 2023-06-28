use itertools::Itertools;
use proconio::input;
use std::{process::exit, string::ToString};

fn contain_ans(ans: &str, t: &[String]) -> bool {
    let x = t.binary_search(&ans.to_string());
    x.is_ok()
}

fn dfs(cur: usize, s: &[String], t: &[String], remain: i64, ans: String) {
    if remain < 0 {
        return;
    }
    if cur == s.len() {
        if ans.len() >= 3 && !contain_ans(&ans, t) {
            println!("{}", ans);
            exit(0);
        }
        return;
    }
    if !ans.is_empty() && (!ans.ends_with('_')) {
        let mut ans_ = ans;
        ans_.push('_');
        dfs(cur, s, t, remain, ans_);
    } else {
        let mut ans_ = ans.clone();
        ans_.push_str(&s[cur]);
        dfs(cur + 1, s, t, remain, ans_);
        if !ans.is_empty() {
            let mut ans_ = ans;
            ans_.push('_');
            dfs(cur, s, t, remain - 1, ans_);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ss: [String; n],
        mut tt: [String; m],
    }
    tt.sort();
    let mut remain = 16;
    for ssi in ss.iter().take(n) {
        remain -= ssi.len() as i64;
    }
    remain -= (n - 1) as i64;
    ss.into_iter()
        .permutations(n)
        .for_each(|perm| dfs(0, &perm, &tt, remain, "".to_string()));
    println!("-1");
}
