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

    // 最初に見た人が pre_ans.0 の場合 と考えて処理
    let mut ans = 0;
    for i in 0..pre_ans.len() {
        let u = i;
        let v = (u + 1) % pre_ans.len(); // 最初に見た人

        let d = (pre_ans[v].0 + M - pre_ans[u].0) % M;
        let d = if d == 0 { M } else { d };
        let e = pre_ans[v].1;
        ans += d * pre_ans[v].1;
    }

    println!("{ans}");
}
