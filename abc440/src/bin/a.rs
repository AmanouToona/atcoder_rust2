use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (mut X, Y): (usize, usize),
    }

    for _ in 0..Y {
        X *= 2;
    }

    println!("{X}");
}
