use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        a: [u64; n]
    }
    let mut answer = 0;
    let mut l = 0;
    let mut r = n - 1;

    while l <= r {
        let pos = (l + r) / 2;
        let cmp_number = a[pos];
        if x < cmp_number {
            r = pos - 1;
        } else if x > cmp_number {
            l = pos + 1;
        } else if x == cmp_number {
            answer = pos;
            break;
        }
    }

    println!("{}", answer + 1);
}
