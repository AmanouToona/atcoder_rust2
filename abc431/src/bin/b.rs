use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        N: usize,
        W: [usize; N],
        Q: usize,
        P: [usize; Q],
    }

    let P: Vec<usize> = P.into_iter().map(|x| x - 1).collect();

    let mut ans = X;
    let mut used = vec![false; N];

    for p in P {
        if used[p] {
            ans -= W[p];
        } else {
            ans += W[p];
        }
        used[p] = !used[p];

        println!("{ans}");
    }
}
