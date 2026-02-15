use proconio::input;
use std::collections::BinaryHeap;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, N): (usize, usize, usize),
        hw: [(usize, usize); N],
    }

    let mut hs: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    let mut ws: BinaryHeap<(usize, usize)> = BinaryHeap::new();

    for (i, &(h, w)) in hw.iter().enumerate() {
        hs.push((h, i));
        ws.push((w, i));
    }

    let mut rest_h = H;
    let mut rest_w = W;
    let mut used: HashSet<usize> = HashSet::new();

    let mut ans = vec![(0, 0); N];
    'outer: while rest_h > 0 && rest_w > 0 {
        while let Some((_, i)) = hs.peek() {
            if used.contains(i) {
                hs.pop();
            } else {
                break;
            }
        }
        while let Some((_, i)) = ws.peek() {
            if used.contains(i) {
                ws.pop();
            } else {
                break;
            }
        }

        if let Some((h, i)) = hs.peek() {
            if *h == rest_h {
                ans[*i] = (H - rest_h, W - rest_w);
                rest_w -= hw[*i].1;
                used.insert(*i);
                continue 'outer;
            }
        }

        if let Some((w, i)) = ws.peek() {
            if *w == rest_w {
                ans[*i] = (H - rest_h, W - rest_w);
                rest_h -= hw[*i].0;
                used.insert(*i);
            }
        }
    }

    for &(h, w) in ans.iter() {
        println!("{} {}", h + 1, w + 1);
    }
}
