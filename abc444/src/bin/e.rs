use proconio::input;
use std::{collections::BTreeMap, ops::Bound::Included, ops::Bound::Unbounded};
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, D): (usize, usize),
        A: [usize; N],
    }

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();

    let mut ans = 0;
    let mut right = 0;
    for left in 0..N {
        // right はまだ加えられていない

        while right < left {
            *map.entry(A[right]).or_default() += 1;
            right += 1;
        }

        while right < N {
            let r = A[right];
            let small = map.range((Unbounded, Included(&r))).next_back();
            if let Some((&k, _)) = small {
                if r - k < D {
                    break;
                }
            }
            let large = map.range((Included(&r), Unbounded)).next();
            if let Some((&k, _)) = large {
                if k - r < D {
                    break;
                }
            }

            *map.entry(r).or_default() += 1;

            right += 1;
        }

        ans += right - left;

        // eprintln!("{left}, {right}, {:?}", map,);
        *map.entry(A[left]).or_default() -= 1;
        if map.get(&A[left]) == Some(&0) {
            map.remove(&A[left]);
        }
    }

    println!("{ans}");
}
