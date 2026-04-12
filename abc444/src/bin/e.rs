use proconio::input;
use std::collections::BTreeSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, D): (usize, usize),
        A: [usize; N],
    }

    let mut set: BTreeSet<usize> = BTreeSet::new();
    let mut ans = 0;
    let mut l = 0;
    for (r, ar) in A.iter().enumerate() {
        while l < r || l < N {
            if let Some(low) = set.range(..A[l]).next_back() {
                if A[l] - low < D {
                    break;
                }
            }

            if let Some(large) = set.range(A[l]..).next() {
                if large - A[l] < D {
                    break;
                }
            }

            set.insert(A[l]);
            l += 1;
        }
        ans += l - r;
        set.remove(ar);
    }

    println!("{ans}");
}
