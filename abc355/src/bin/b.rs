use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u16;n],
        b: [u16;m],
    }

    let mut sorted_list_c = a.clone();
    sorted_list_c.extend(b);
    sorted_list_c.sort();

    let mut result = "No";
    if n > 1 {
        for idx in 0..(n + m) {
            if idx + 1 < (n + m) {
                if a.contains(&sorted_list_c[idx]) && a.contains(&sorted_list_c[idx + 1]) {
                    result = "Yes";
                    break;
                }
            }
        }
    }
    println!("{}", result);
}
