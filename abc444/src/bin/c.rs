use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }

    A.sort();

    let mut ans = HashSet::new();
    let mut set: HashSet<usize> = HashSet::new();
    for &a in A.iter() {
        set.insert(a);
    }
    if set.len() == 1 {
        ans.insert(set.iter().next().unwrap().clone());
    }

    if N % 2 == 0 {
        let mut set: HashSet<usize> = HashSet::new();
        for i in 0..A.len() {
            let l = i;
            let r = A.len() - 1 - i;
            set.insert(A[l] + A[r]);
        }

        if set.len() == 1 {
            ans.insert(set.iter().next().unwrap().clone());
        }
    }

    let mut r = 0;
    for (i, &a) in A.iter().enumerate() {
        if a != A[N - 1] {
            r = i;
        }
    }

    if r % 2 == 1 {
        let mut set = HashSet::new();
        set.insert(A[N - 1]);
        for i in 0..r {
            set.insert(A[i] + A[r - i]);
        }
        if set.len() == 1 {
            ans.insert(set.iter().next().unwrap().clone());
        }
    }

    let mut a = Vec::new();
    for &i in ans.iter() {
        a.push(i);
    }
    a.sort();
    let ans: String = a.iter().join(" ");
    println!("{ans}");
}
