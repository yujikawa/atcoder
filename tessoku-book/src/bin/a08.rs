use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: [[i64; w]; h],
        q: usize,
        position: [(usize, usize, usize, usize); q],
    }

    // 横の累積和
    for i in 0..h {
        for j in 1..w {
            x[i][j] = x[i][j] + x[i][j - 1]
        }
    }

    // 縦の累積和
    for i in 1..h {
        for j in 0..w {
            x[i][j] = x[i][j] + x[i - 1][j]
        }
    }

    for (a, b, c, d) in position {
        if a == 1 && b == 1 {
            println!("{}", x[c - 1][d - 1]);
        } else if a > 1 && b == 1 {
            let answer = x[c - 1][d - 1] - x[a - 2][d - 1];
            println!("{}", answer);
        } else if a == 1 && b > 1 {
            let answer = x[c - 1][d - 1] - x[c - 1][b - 2];
            println!("{}", answer);
        } else if a > 1 && b > 1 {
            let answer = x[c - 1][d - 1] + x[a - 2][b - 2] - x[a - 2][d - 1] - x[c - 1][b - 2];
            println!("{}", answer);
        }
    }

    // println!("{:?}", x);
}
