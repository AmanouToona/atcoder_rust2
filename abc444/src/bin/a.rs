use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: Chars,
    }

    if N[0] == N[1] && N[1] == N[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
