use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
    }

    let a_sum: usize = A.iter().sum();

    for a in A.iter() {
        if a_sum - *a == M {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
