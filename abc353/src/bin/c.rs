use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [u32; n]
    }
    let mut result = 0;
    let mut n_count = 0;
    let t = 100000000;
    let mut sorted_list = a.clone();
    sorted_list.sort_by(|a, b| b.cmp(a));
    for i in 0..n {
        result = result + a[i];
    }

    for i in 0..n {
        
        // for j in (i + 1)..n {
        //     if (sorted_list[i] + sorted_list[j]) >= t {
        //         n_count = n_count + 1;
        //     } else {
        //         break;
        //     }
        // }
    }

    let fix = (n as u32 - 1) * result - t * n_count;
    println!("{}", fix);
}
