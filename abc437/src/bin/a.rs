use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B): (usize, usize),
    }

    println!("{}", A * 12 + B);
}
