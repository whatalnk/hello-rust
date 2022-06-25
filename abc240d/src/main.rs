use proconio::input;

struct S {
    key: usize,
    cnt: usize,
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut v: Vec<S> = vec![];
    for i in 0..n {
        if let Some(x) = v.pop() {
            if x.key == a[i] {
                let mut cur = S {
                    key: a[i],
                    cnt: x.cnt,
                };
                if cur.key == cur.cnt + 1 {
                    ans -= cur.cnt;
                } else {
                    cur.cnt += 1;
                    v.push(cur);
                    ans += 1;
                }
            } else {
                v.push(x);
                let cur = S { key: a[i], cnt: 1 };
                v.push(cur);
                ans += 1;
            }
        } else {
            let cur = S { key: a[i], cnt: 1 };
            v.push(cur);
            ans += 1;
        }
        println!("{}", ans);
    }
}
