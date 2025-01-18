use proconio::input;
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize
    }

    let mut arr = (1..=n).collect::<Vec<usize>>();
    let slice = &mut arr[l - 1..r];
    slice.reverse();
    for i in 1..=n {
        if i == n {
            print!("{}", arr[i - 1]);
        } else {
            print!("{} ", arr[i - 1]);
        }
    }

}
