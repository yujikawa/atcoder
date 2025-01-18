use proconio::input;
fn main() {
    input! {
        n: usize,
        t: usize,
        a: [u16; t],
    }

    // Create bingo NxN N*(i-1)+j
    let mut bingo = vec![vec![0; n]; n];
}

fn calc(i: u16, j: u16, n: usize) -> u16 {
    return N*(i-1)+j
}
