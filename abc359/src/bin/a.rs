use proconio::input;
fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }

    let mut result = 0;

    for name in s {
        if name == "Takahashi" {
            result += 1;
        }
    }

    println!("{}", result);
}
