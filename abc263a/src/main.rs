use proconio::input;

fn main() {
    input! {
        c: [usize; 5]
    }
    let mut v = vec![0; 14];
    for i in 0..5 {
        v[c[i]] += 1;
    }
    let mut two = false;
    let mut three = false;
    for vi in v.iter().take(14) {
        if vi == &2 {
            two = true;
        }
        if vi == &3 {
            three = true;
        }
    }
    if two && three {
        println!("Yes");
    } else {
        println!("No");
    }
}
