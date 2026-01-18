use ac_library::FenwickTree;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut cumsum = n; // 0-index, [ , )
    let mut seg = FenwickTree::new(n * 2 + 1, 0i64);
    seg.add(cumsum, 1);
    let mut ans = 0;
    for (i, &c) in s.iter().enumerate() {
        match c {
            'A' => {
                cumsum += 1;
            }
            'B' => {
                cumsum -= 1;
            }
            'C' => {}
            _ => {
                panic!()
            }
        }

        ans += seg.sum(..cumsum);
        seg.add(cumsum, 1);
    }

    println!("{ans}");
}
