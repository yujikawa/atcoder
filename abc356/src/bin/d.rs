use proconio::input;
fn main() {
    input! {
        n: u64,
        m: u64,
    }
    let mut result = 0;
    for i in 0..=n {
        result += 1;
        // print!("{}", i);
    }

    println!("{}", result);
}
