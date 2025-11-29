use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize,
    }

    for i in 0..N {
        if i < M {
            println!("OK");
        } else {
            println!("Too Many Requests")
        }
    }
}
