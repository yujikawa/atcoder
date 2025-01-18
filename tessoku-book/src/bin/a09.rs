use std::vec;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        // (a, b, c, d)
        pos: [(usize, usize, usize, usize); n],
    }

    // h x w = 4 x 3 = [[0,0,0], [0,0,0], [0,0,0], [0,0,0]]
    let mut matrix = vec![vec![0; w+10]; h+10];

    // index = 1
    // (a, b) (c+1, d+1) = +1
    // (a, d+1) (c+1, b) = -1

    // index = 0
    // (a-1, b-1) (c, d) = +1
    // (a-1, d) (c, b-1) = -1

    for (a, b, c, d) in pos {
        matrix[a - 1][b - 1] += 1;
        matrix[c][d] += 1;
        matrix[a - 1][d] -= 1;
        matrix[c][b - 1] -= 1;
    }

    for i in 0..h {
        for j in 1..w {
            matrix[i][j] = matrix[i][j] + matrix[i][j - 1]
        }
    }
    for i in 1..h {
        for j in 0..w {
            matrix[i][j] = matrix[i][j] + matrix[i - 1][j]
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", matrix[i][j]);
            if j != (w - 1) {
                print!(" ");
            }
        }
        println!(); // 各行の後に改行
    }
}
