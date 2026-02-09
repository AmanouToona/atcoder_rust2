use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            (N, M): (usize, usize),
            A: [usize; N],
        }

        let mut left = 0;
        let mut right = A[N - 1];
    }
}
