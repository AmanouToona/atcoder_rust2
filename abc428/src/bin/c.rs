use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }

    let mut s = Vec::new();
    let mut sum = 0;
    let mut cnt_under0 = 0;

    for _ in 0..Q {
        input! {q: usize}
        if q == 1 {
            input! {c: char};
            let c = if c == '(' { 1 } else { -1 };
            sum += c;
            s.push(c);
            if sum < 0 {
                cnt_under0 += 1;
            }
        } else {
            let c = s.pop().unwrap();
            if sum < 0 {
                cnt_under0 -= 1;
            }
            sum -= c;
        }

        if sum == 0 && cnt_under0 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
