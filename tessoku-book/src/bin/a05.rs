use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }
    let mut count: u64 = 0;
    for i in 1..=n {
        for j in 1..=n {
            let result = k - i - j;
            if result <= n && result >= 1 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
