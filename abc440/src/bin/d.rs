use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        mut A: [usize; N],
    }

    A.sort();
    A.dedup();

    for _ in 0..Q {
        input! {
            (x, y): (usize, usize),
        }
        let count_less = A.partition_point(|&v| v < x);

        let mut right: usize = 10_000_000_000;
        let mut left: usize = x - 1;

        while right - left > 1 {
            let mid = (right + left) / 2;
            let count: usize = A.partition_point(|&v| v <= mid);

            if mid - (x - 1) - (count - count_less) >= y {
                right = mid;
            } else {
                left = mid;
            }
        }
        println!("{right}");
    }
}
