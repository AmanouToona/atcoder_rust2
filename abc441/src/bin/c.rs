use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K, X): (usize, usize, usize),
        mut A: [usize; N]
    }

    A.sort();
    let sake: Vec<usize> = A.iter().take(K).cloned().collect();

    let mut ans = N - K;
    let mut sum = 0;
    for &s in sake.iter() {
        sum += s;
        ans += 1;
        if sum >= X {
            break;
        }
    }

    if sum < X {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
