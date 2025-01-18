use proconio::input;

fn main() {
    input! {
        n: usize, // n本の鍵
        m: usize, // m回の実験
        k: usize, // k本以上差し込んだときに開く
    }
    let mut input_data = Vec::<(usize, Vec<usize>, String)>::new();
    for _ in 0..m {
        input! {
            c: usize,
            a: [usize; c],
            r: String,
        }

        input_data.push((c, a, r))
    }

    // ｍ回のテストパターンが通った組み合わせの回数を数える変数
    let mut answer = 0;
    // 鍵の組み合わせパターンを回す
    for key_patern in 0..(1 << n) as usize {
        // m回の実験結果を確認する
        let mut is_ok = true;
        for (_, a, r) in &input_data {
            let mut compare_key_patern = 0;
            // テストの鍵で組み合わせバイナリを作成(ex: 101, 011)
            for key in a {
                compare_key_patern |= 1 << key - 1;
            }

            let ones_count = (compare_key_patern & key_patern).count_ones();
            if !(r == "o" && ones_count >= k as u32) && !(r == "x" && ones_count < k as u32) {
                is_ok = false;
            }
        }

        // テストを満たすパターンが存在した場合にカウント
        if is_ok {
            answer += 1;
        }
    }

    println!("{}", answer);
}
