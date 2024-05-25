use proconio::input;
fn main() {
    input! {
        n: u32, // 待機列数
        k: u32, // アトラクションの乗れる人数
        a: [u32; n]
    }

    // アトラクションを動かした回数
    let mut run_count = 1;
    let mut availability = k;
    for num_of_customer in a {
        if availability >= num_of_customer {
            availability = availability - num_of_customer;
        } else {
            run_count += 1;
            availability = k;
            availability = availability - num_of_customer;
        }
    }
    println!("{}", run_count);
}
