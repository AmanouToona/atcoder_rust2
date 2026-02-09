use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }

    A.sort();

    let mut ans = Vec::new();
    // 最長の物以外で最長のものの長さにできるか？
    let shorter = A
        .iter()
        .filter(|&x| *x != A[N - 1])
        .cloned()
        .collect::<Vec<usize>>();
    let mut is_answer = shorter.len() % 2 == 0;
    for (&i, &j) in shorter.iter().zip(shorter.iter().rev()) {
        if i + j != A[N - 1] {
            is_answer = false;
        }
    }
    if is_answer {
        ans.push(A[N - 1]);
    }

    // 全てのじゃがりこを使うことはできるか？
    let mut is_answer = A.len() % 2 == 0;
    for (&i, &j) in A.iter().zip(A.iter().rev()) {
        if i + j != A[0] + A[N - 1] {
            is_answer = false;
        }
    }
    if is_answer {
        ans.push(A[0] + A[N - 1]);
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
