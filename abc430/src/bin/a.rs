use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C, D): (usize, usize, usize, usize),
    }

    if C < A {
        println!("No");
        return;
    }

    if D < B {
        println!("Yes");
        return;
    }

    println!("No");
}
