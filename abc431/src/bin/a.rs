use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, B): (usize, usize),
    }
    println!("{}", H.saturating_sub(B));
}
