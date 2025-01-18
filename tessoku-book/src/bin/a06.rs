use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        term: [[usize; 2]; q],
        // r: [usize; q],
    }

    // 累積和の計算
    let mut s: Vec<u64> = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }
    // println!("{:?}", s);
    for t in term {
        let start = t[0] - 1;
        let end = t[1];
        let result = s[end] - s[start];
        println!("{}", result);
    }
}
