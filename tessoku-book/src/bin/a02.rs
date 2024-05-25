use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        a: [i32;n],
    };

    let mut result = "No";

    for i in a {
        if (i == x) {
            result = "Yes";
            break;
        }
    }

    println!("{}", result);
}
