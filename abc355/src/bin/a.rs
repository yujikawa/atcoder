use proconio::input;
fn main() {
    input! {
        a: u8,
        b: u8,
    }

    let result = match (a, b) {
        (1, 2) => 3,
        (2, 1) => 3,
        (2, 3) => 1,
        (3, 2) => 1,
        (1, 3) => 2,
        (3, 1) => 2,
        _ => -1,
    };

    println!("{}", result);
}
