use proconio::input;
fn main() {
    input! {d: usize, n: usize, u: [(usize, usize); n]}
    let mut s: Vec<i64> = vec![0; d + 10];

    for (start, end) in u {
        s[start] += 1;
        s[end + 1] -= 1;
        // println!("{} {} {:?}", start, end, s);
    }

    // println!("{:?}", s);

    let mut result = vec![0; d + 1];
    for i in 1..=d {
        if result.len() > d {
            result[i] = result[i - 1] + s[i];
        }
    }
    for i in 1..=d {
        println!("{}", result[i]);
    }
}
