use proconio::input;
fn main() {
    input! {
        n: u32,
        h: [u32; n],
    }
    let first = h[0];
    let mut index = 1;
    let mut is_higher_num = false;

    for v in h {
        if v > first {
            is_higher_num = true;
            break;
        }
        index = index + 1;
    }

    if index == 1 || is_higher_num == false {
        println!("-1");
    } else {
        println!("{}", index);
    }
}
