use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [u32; n]
    }
    let mut result = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            result = result + func(a[i], a[j]);
        }
    }
    println!("{}", result);
}

fn func(x: u32, y: u32) -> u32 {
    (x + y) % 10_u32.pow(8)
}
