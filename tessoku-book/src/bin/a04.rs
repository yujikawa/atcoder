use proconio::input;

fn main() {
    input! {n: u32}
    let a: u32 = 2;
    for i in (0..10).rev() {
        let bunbo = a.pow(i as u32);
        let result = (n / bunbo) % 2;
        print!("{}", result);
    }
}
