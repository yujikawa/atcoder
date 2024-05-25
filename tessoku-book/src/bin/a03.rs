use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
        p: [i32;n],
        q: [i32;n],
    };

    let mut result = "No";
    for pn in &p {
        for qn in &q {
            if k == pn + qn {
                result = "Yes";
                break;
            }
        }
    }
    println!("{}", result);
}
