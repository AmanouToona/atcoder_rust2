use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, C): (usize, usize, usize),
        A: [usize; N]
    }

    let mut pos: HashMap<usize, usize> = HashMap::new();
    for a in A.iter() {
        *pos.entry(*a).or_default() += 1;
    }

    let mut pos = pos.into_iter().collect::<Vec<(usize, usize)>>();
    pos.sort_by(|a, b| a.0.cmp(&b.0));

    let mut iter = pos.iter().cycle();
    let mut sum = 0;
    let mut people = VecDeque::new();
    let mut pre_ans = Vec::new();

    for &p in pos.iter() {
        while sum < C {
            let nxt = iter.next().unwrap();
            sum += nxt.1;
            people.push_back(*nxt);
        }

        pre_ans.push((p.0, sum));
        let v = people.pop_front().unwrap();
        sum -= v.1;
    }

    let mut ans = 0;
    for i in 0..pre_ans.len() {
        if i == 0 {
            ans += pre_ans[i].0 * pre_ans[i].1;
        } else {
            let from = pre_ans[i - 1];
            let to = pre_ans[i];
            ans += from.1 * (to.0 - from.0);
        }
    }
    ans += (M - pre_ans.last().unwrap().0) * pre_ans.last().unwrap().1;

    println!("{ans}");
}
