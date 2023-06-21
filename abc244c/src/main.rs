use proconio::input;
use proconio::source::line::LineSource;
use std::io::{stdin, BufReader};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize
    }
    let mut checked = vec![false; 2 * n + 2];
    loop {
        for (i, checkedi) in checked.iter_mut().enumerate().skip(1).take(2 * n + 1) {
            if !*checkedi {
                println!("{}", i);
                *checkedi = true;
                break;
            }
        }
        input! {
            from &mut source,
            k: usize
        }
        if k == 0 {
            break;
        }
        checked[k] = true;
    }
}
