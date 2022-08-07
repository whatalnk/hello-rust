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
    for i in 0..14 {
        if v[i] == 2 {
            two = true;
        }
        if v[i] == 3 {
            three = true;
        }
    }
    if two && three {
        println!("Yes");
    } else {
        println!("No");
    }
}
