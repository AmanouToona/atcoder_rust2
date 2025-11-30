use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (S, A, B, X): (usize, usize, usize, usize)
    }

    let set = X / (A + B);
    let run = set * A + A.min(X - (A + B) * set);
    let ans = S * run;
    println!("{ans}");
}
