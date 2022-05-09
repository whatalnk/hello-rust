use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }
    let mut r1 = Vec::new();
    let mut r2 = Vec::new();
    for i in 0..n {
        if (i + 1) % 2 != 0 {
            r1.push(".".repeat(b));
            r2.push("#".repeat(b));
        } else {
            r1.push("#".repeat(b));
            r2.push(".".repeat(b));
        }
    }
    let r1s: String = vec![r1.join(""); a].join("\n");
    let r2s: String = vec![r2.join(""); a].join("\n");
    for i in 0..n {
        if (i + 1) % 2 != 0 {
            println!("{}", r1s);
        } else {
            println!("{}", r2s);
        }
    }
}
