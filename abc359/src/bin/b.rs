use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [u8; 2*n],
    }

    let mut result = 0;

    for i in 0..2 * n {
        if i + 2 < 2 * n {
            if a[i] == a[i + 2] {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
