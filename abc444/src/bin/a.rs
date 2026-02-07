use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }

    if (N / 100 == (N % 100) / 10) && (N % 100) / 10 == (N % 10) {
        println!("Yes");
    } else {
        println!("No")
    }
}
