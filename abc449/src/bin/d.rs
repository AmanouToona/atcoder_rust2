#![allow(non_snake_case)]
use num::Integer;
use proconio::input;
fn main() {
    input! {
        (L, R, D, U): (i64,i64,i64,i64)
    }
    let mut ans = 0;
    // |x| >= |y|
    for x in L..=R {
        if x.is_even() {
            if D > x.abs() || U < -x.abs() {
                continue;
            }
            let d = (-x.abs()).max(D);
            let u = x.abs().min(U);
            ans += u - d + 1;
        }
    }

    // |x| < |y|
    for y in D..=U {
        if y.is_even() {
            if L >= y.abs() || R <= -y.abs() {
                continue;
            }
            let l = (-y.abs() + 1).max(L);
            let r = (y.abs() - 1).min(R);
            ans += 0.max(r - l + 1);
        }
    }

    println!("{ans}");
}
