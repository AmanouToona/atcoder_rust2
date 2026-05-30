use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N == 1 {
        println!("Yes");
        return;
    }

    let mut seen = vec![false; 10000];
    seen[N] = true;

    let mut n = N;
    loop {
        let mut nxt = 0;
        for i in n.to_string().chars() {
            let i = i.to_digit(10).unwrap() as usize;
            nxt += i * i;
        }
        if seen[nxt] {
            println!("No");
            return;
        }
        seen[nxt] = true;
        if nxt == 1 {
            println!("Yes");
            return;
        }
        n = nxt;
    }
}
