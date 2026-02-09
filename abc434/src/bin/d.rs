use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let size = 2000;

    let mut cumsum = vec![vec![0i64; size + 2]; size + 2];
    let mut clouds = Vec::new();
    for _ in 0..N {
        input! {
            (u, d, l, r): (usize, usize, usize, usize),
        }
        cumsum[u][l] += 1;
        cumsum[u][r + 1] -= 1;
        cumsum[d + 1][l] -= 1;
        cumsum[d + 1][r + 1] += 1;
        clouds.push((u, d, l, r));
    }

    // 覆っている雲の個数の累積話を完成
    for i in 1..=size {
        for j in 1..=size {
            cumsum[i][j] += cumsum[i][j - 1] + cumsum[i - 1][j] - cumsum[i - 1][j - 1];
        }
    }

    // 雲が1つのみの領域の累積和と、雲が 0 個の領域を保存
    let mut no_cloud = 0;
    let mut cumsum1 = vec![vec![0i64; size + 2]; size + 2];

    for i in 1..=size {
        for j in 1..=size {
            if cumsum[i][j] == 0 {
                no_cloud += 1;
            } else if cumsum[i][j] == 1 {
                cumsum1[i][j] = 1;
            }
        }
    }

    // 雲が1つのみの領域の累積話を完成
    for i in 1..=size {
        for j in 1..=size {
            cumsum1[i][j] += cumsum1[i][j - 1] + cumsum1[i - 1][j] - cumsum1[i - 1][j - 1];
        }
    }

    // 回答
    for &(u, d, l, r) in clouds.iter() {
        let one = cumsum1[d][r] - cumsum1[d][l - 1] - cumsum1[u - 1][r] + cumsum1[u - 1][l - 1];
        let ans = no_cloud + one;
        println!("{ans}");
    }
}
