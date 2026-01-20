use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: [usize; N]
    }

    let mut T: Vec<(usize, usize)> = T.into_iter().enumerate().map(|(x, y)| (x + 1, y)).collect();
    T.sort_by(|x, y| x.1.cmp(&y.1));

    let ans: String = T.iter().take(3).map(|x| x.0).join(" ");
    println!("{ans}");
}
