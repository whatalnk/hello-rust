use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize
    }
    let mut m = vec![vec![0; 15]; 15];
    let st = vec![1usize, 3, 5, 7];

    for i in 0..4 {
        // right
        let s = st[i];
        let l = 15 - (s - 1) * 2;
        let rr = s - 1;
        for cc in 0..l {
            m[rr][s - 1 + cc] = 1;
        }
        // left
        let rr = 15 - s;
        for cc in 0..l {
            m[rr][s - 1 + cc] = 1;
        }
        // down
        let cc = 15 - s;
        for rr in 0..l {
            m[s - 1 + rr][cc] = 1;
        }
        // up
        let cc = s - 1;
        for rr in 0..l {
            m[s - 1 + rr][cc] = 1;
        }
    }
    if m[r - 1][c - 1] == 0 {
        println!("white");
    } else {
        println!("black");
    }
}
