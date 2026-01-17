use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (P, Q) : (usize, usize),
        (X, Y) : (usize, usize),
    }

    if X >= P && X <= P + 99 && Y >= Q && Y <= Q + 99 {
        println!("Yes");
    } else {
        println!("No");
    }
}
