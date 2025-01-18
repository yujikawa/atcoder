use std::sync::TryLockResult;

use proconio::input;
fn main() {
    input! {
        n: usize, // 品数
        m: usize, // 栄養素種類数
        a: [u64; m], // 栄養素摂取目安
        mut x: [[u64; m]; n], // n品のそれぞれmの数の栄養素の摂取量
    }

    for i in 1..n {
        for j in 0..m {
            x[i][j] = x[i][j] + x[i - 1][j];
        }
    }

    let mut result = true;

    // println!("{:?}", x);
    for i in 0..m {
        if x[n - 1][i] < a[i] {
            result = false;
            break;
        }
    }

    if result {
        println!("Yes");
    } else {
        println!("No");
    }

}
