use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            (N, W): (usize, usize),
            C: [usize; N],
        }

        let mut cost = vec![0; 2 * W];
        for (i, &c) in C.iter().enumerate() {
            cost[i % (2 * W)] += c;
        }

        let cost: Vec<usize> = cost.iter().cycle().take(4 * W).cloned().collect();

        let mut sum: usize = cost.iter().take(W).sum();
        let mut ans = sum;
        for i in W..4 * W {
            let left = i - W;
            sum -= cost[left];
            sum += cost[i];

            ans = ans.min(sum);
        }

        // println!("{:?}", cost);
        println!("{ans}");
    }
}
