#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
    }

    let mut next_to_id = Vec::from_iter(0..N);
    let mut id_to_nest = Vec::from_iter(0..N);
    let mut pegion_to_id = Vec::from_iter(0..N);

    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                input! {(a, b): (usize, usize)}
                let a = a - 1;
                let b = b - 1;

                pegion_to_id[a] = next_to_id[b];
            }
            2 => {
                input! {(a, b): (usize, usize)}
                let a = a - 1;
                let b = b - 1;

                let frm_a = next_to_id[a];
                let frm_b = next_to_id[b];
                next_to_id.swap(a, b);
                id_to_nest.swap(frm_a, frm_b);
            }
            3 => {
                input! {a: usize}
                let a = a - 1;
                let ans = id_to_nest[pegion_to_id[a]] + 1;
                println!("{ans}");
            }
            _ => {
                panic!("")
            }
        }
    }
}
