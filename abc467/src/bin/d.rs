#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            px: i128,
            py: i128,
            qx: i128,
            qy: i128,
            rx: i128,
            ry: i128,
            sx: i128,
            sy: i128,
        };

        let a1 = 2 * (qx - px);
        let b1 = 2 * (qy - py);
        let c1 = qx * qx + qy * qy - px * px - py * py;

        let a2 = 2 * (sx - rx);
        let b2 = 2 * (sy - ry);
        let c2 = sx * sx + sy * sy - rx * rx - ry * ry;

        // eprintln!("{a1} {b1} {c1} {a2} {b2} {c2}");
        if a1 * b2 == a2 * b1 && (a1 * c2 != a2 * c1 || b1 * c2 != b2 * c1) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
