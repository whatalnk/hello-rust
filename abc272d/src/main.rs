use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut d = vec![vec![-1; n + 1]; n + 1];
    let mut moveto = vec![];
    for i in 0..=n {
        for j in i..=n {
            if i * i + j * j == m {
                moveto.push((i, j));
            }
        }
    }
    d[1][1] = 0;
    let mut q = VecDeque::new();
    q.push_back((1, 1));
    loop {
        if let Some(p) = q.pop_front() {
            for i in 0..moveto.len() {
                for (s1, s2) in vec![(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    let ni = (p.0 as isize) + (moveto[i].0 as isize) * s1;
                    let nj = (p.1 as isize) + (moveto[i].1 as isize) * s2;
                    if ni > 0
                        && ni <= n as isize
                        && nj > 0
                        && nj <= n as isize
                        && d[ni as usize][nj as usize] < 0
                    {
                        d[ni as usize][nj as usize] = d[p.0][p.1] + 1;
                        q.push_back((ni as usize, nj as usize));
                    }
                    let ni = (p.0 as isize) + (moveto[i].1 as isize) * s1;
                    let nj = (p.1 as isize) + (moveto[i].0 as isize) * s2;
                    if ni > 0
                        && ni <= n as isize
                        && nj > 0
                        && nj <= n as isize
                        && d[ni as usize][nj as usize] < 0
                    {
                        d[ni as usize][nj as usize] = d[p.0][p.1] + 1;
                        q.push_back((ni as usize, nj as usize));
                    }
                }
            }
        } else {
            break;
        }
    }
    for i in 1..=n {
        println!(
            "{}",
            &d[i][1..]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
