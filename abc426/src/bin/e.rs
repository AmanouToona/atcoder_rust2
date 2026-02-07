#![allow(non_snake_case)]
use proconio::input;

fn distance(t: (f64, f64), a: (f64, f64)) -> f64 {
    ((t.0 - a.0).powf(2.) + (t.1 - a.1).powf(2.)).sqrt()
}

fn get_position(s: (f64, f64), g: (f64, f64), t: f64) -> (f64, f64) {
    let d = ((g.0 - s.0).powf(2.) + (g.1 - s.1).powf(2.)).sqrt();
    let x = s.0 + (g.0 - s.0) / d * t.min(d);
    let y = s.1 + (g.1 - s.1) / d * t.min(d);

    (x, y)
}

fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            (Tsx, Tsy, Tgx, Tgy): (f64, f64, f64, f64),
            (Asx, Asy, Agx, Agy): (f64, f64, f64, f64),
        }

        // 3分探索
        let mut left = 0.;
        let mut right = 300.;
        // left c1 c2 right;
        for _ in 0..500 {
            let d = (right - left) / 3.;
            let c1 = left + d;
            let c2 = c1 + d;

            let tc1 = get_position((Tsx, Tsy), (Tgx, Tgy), c1);
            let ac1 = get_position((Asx, Asy), (Agx, Agy), c1);
            let tc2 = get_position((Tsx, Tsy), (Tgx, Tgy), c2);
            let ac2 = get_position((Asx, Asy), (Agx, Agy), c2);

            if distance(tc1, ac1) > distance(tc2, ac2) {
                left = c1;
            } else {
                right = c2;
            }
        }
        let tp = get_position((Tsx, Tsy), (Tgx, Tgy), left);
        let ap = get_position((Asx, Asy), (Agx, Agy), left);
        let ans = distance(tp, ap);
        println!("{ans}");
    }
}
