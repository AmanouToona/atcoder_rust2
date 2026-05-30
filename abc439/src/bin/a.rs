use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: u32
    }

    let ans = 2_i128.pow(N) - 2_i128 * N as i128;
    println!("{ans}");
}
