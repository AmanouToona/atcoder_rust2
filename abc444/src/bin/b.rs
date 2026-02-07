use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
    }

    let mut ans = 0;

    for i in 1..=N {
        let i: usize = i
            .to_string()
            .chars()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .sum();
        if i == K {
            ans += 1;
        }
    }

    println!("{ans}");
}
