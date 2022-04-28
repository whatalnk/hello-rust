use proconio::input;
use proconio::marker::Chars;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    win: i32,
    no: usize,
}

fn gcp(h1: char, h2: char) -> i32 {
    let res = match (h1, h2) {
        ('G', 'C') | ('C', 'P') | ('P', 'G') => 1,
        ('G', 'G') | ('C', 'C') | ('P', 'P') => 0,
        _ => -1,
    };
    return res;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2*n]
    }
    let mut persons = vec![];
    for i in 0..(2 * n) {
        persons.push(Person { win: 0, no: i });
    }
    for i in 0..m {
        for k in 0..n {
            let p1 = &persons[2 * k];
            let p2 = &persons[2 * k + 1];
            let h1 = a[p1.no][i];
            let h2 = a[p2.no][i];
            if gcp(h1, h2) > 0 {
                persons[2 * k].win -= 1;
            } else if gcp(h1, h2) < 0 {
                persons[2 * k + 1].win -= 1
            }
        }
        persons.sort();
    }
    for i in 0..(2 * n) {
        println!("{}", persons[i].no + 1);
    }
}
