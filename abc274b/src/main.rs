use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut x = vec![0; w];
    for ci in c.iter().take(h) {
        for (j, cij) in ci.iter().enumerate().take(w) {
            if cij == &'#' {
                x[j] += 1;
            }
        }
    }
    println!(
        "{}",
        x.iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
