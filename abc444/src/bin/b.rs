use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
    }

    let mut ans = 0;

    for i in 1..=N {
        if i.to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .sum::<usize>()
            == K
        {
            ans += 1;
        }
    }

    println!("{ans}");
}
