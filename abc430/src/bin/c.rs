use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, A, B): (usize, usize, usize),
        S: Chars,
    }

    let mut count_a = 0;
    let mut count_b = 0;
    let mut ans: i64 = 0;

    let mut r_a = 0;
    let mut r_b = 0;

    for l in 0..N {
        // [l, r]
        while r_a <= l || (count_a < A && r_a < N) {
            if S[r_a] == 'a' {
                count_a += 1;
            }
            r_a += 1;
        }

        if count_a < A {
            break;
        }

        // [l, r)
        while r_b <= l || (count_b < B && r_b < N) {
            if S[r_b] == 'b' {
                count_b += 1;
            }
            r_b += 1;
        }

        if r_b == N && count_b < B {
            r_b += 1;
        }

        ans += r_b.saturating_sub(r_a) as i64;

        if S[l] == 'a' {
            count_a -= 1;
        } else {
            count_b -= 1;
        }
    }

    println!("{}", ans);
}
