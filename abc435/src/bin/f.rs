use proconio::input;

fn dfs(u: usize, left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let end = left.len();
    let r_cost = if right[u] != end {
        u.abs_diff(right[u]) + dfs(right[u], left, right)
    } else {
        0
    };

    let l_cost = if left[u] != end {
        u.abs_diff(left[u]) + dfs(left[u], left, right)
    } else {
        0
    };

    r_cost.max(l_cost)
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }

    let mut left = vec![N; N];
    let mut right = vec![N; N];
    let mut q: Vec<(usize, usize)> = Vec::new();

    for (i, &p) in P.iter().enumerate() {
        // pop 処理
        while let Some((last_p, last_idx)) = q.last() {
            if *last_p < p {
                left[i] = *last_idx;
                q.pop();
            } else {
                break;
            }
        }

        // push 処理
        if let Some((_, last)) = q.last() {
            right[*last] = i;
        }
        q.push((p, i));
    }

    let root = q[0].1;
    let ans = dfs(root, &left, &right);

    println!("{ans}");
}
