#![allow(non_snake_case)]
use proconio::input;

fn cross(a: (f64, f64), b: (f64, f64)) -> f64 {
    a.0 * b.1 - a.1 * b.0
}

fn dot(a: (f64, f64), b: (f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1
}

fn sub(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 - b.0, a.1 - b.1)
}

fn add(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 + b.0, a.1 + b.1)
}

fn times(a: (f64, f64), p: f64) -> (f64, f64) {
    (a.0 * p, a.1 * p)
}

fn norm(a: (f64, f64)) -> f64 {
    (a.0.powi(2) + a.1.powi(2)).sqrt()
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    norm(sub(a, b))
}

fn distance_line_dot(a: (f64, f64), b: (f64, f64), p: (f64, f64)) -> f64 {
    if a.0 == b.0 && a.1 == b.1 {
        return distance(a, p);
    }

    let ab = sub(b, a);
    let ap = sub(p, a);
    let ba = sub(a, b);
    let bp = sub(p, b);

    if dot(ap, ab) < 0. {
        // 線分の端点aが最短
        return distance(a, p);
    }
    if dot(bp, ba) < 0. {
        return distance(b, p);
    }

    // 線分への垂線が最短距離
    let s = (cross(ab, ap) / 2.).abs();
    s / distance(a, b) * 2.
}

fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            (mut Ts, mut Tg): ((f64, f64), (f64, f64)),
            (mut As, mut Ag): ((f64, f64), (f64, f64)),
        }

        if distance(Ts, Tg) < distance(As, Ag) {
            std::mem::swap(&mut Ts, &mut As);
            std::mem::swap(&mut Tg, &mut Ag);
        }

        let T = sub(Tg, Ts);
        let A = sub(Ag, As);
        // phase1
        // 原点から線分への距離
        let T_split: (f64, f64) = times(T, distance(As, Ag) / distance(Ts, Tg));
        let S = sub(Ts, As);
        let G = sub(add(Ts, T_split), Ag);
        let mut ans = distance_line_dot(S, G, (0., 0.));

        // phase2
        // Ag から線分への距離
        let S = add(Ts, T_split);
        let G = Tg;
        ans = ans.min(distance_line_dot(S, G, Ag));

        println!("{ans}");
    }
}
