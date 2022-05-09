use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }
    let mut r1 = vec![""];
    let mut r2 = vec![""];
    let white = ".".repeat(b);
    let black = "#".repeat(b);
    for i in 0..n {
        if (i + 1) % 2 != 0 {
            r1.push(&white);
            r2.push(&black);
        } else {
            r1.push(&black);
            r2.push(&white);
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
