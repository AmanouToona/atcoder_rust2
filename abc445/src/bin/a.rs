use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input!{
        S: Chars,
    }

    if S[0] == S[S.len() - 1] {
        println!("Yes");
    } else {
        println!("No")
    }
}
