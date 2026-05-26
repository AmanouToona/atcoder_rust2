#![allow(non_snake_case)]
use proconio::input;

/*
特徴
- 必ずある長方形の頂点を原点に触れさせることが可能

- dfs?
- ある長方形を x, y に張り付かせる？
*/

struct Solve {
    x: usize,
    y: usize,
    bx: Vec<usize>,
}

impl Solve {
    fn new(x: usize, y: usize, a: usize, b: usize, c: usize) -> Self {
        Solve {
            x,
            y,
            bx: vec![a, b, c],
        }
    }

    fn can_x(&self, i: usize) -> bool {
        //! 箱 i を x 軸に張り付かせる

        if self.bx[i].div_ceil(self.x) >= self.y {
            return false;
        }

        let res_y = self.y - self.bx[i].div_ceil(self.x);
        // 使える領域は x, res_y の四角形

        // res_y 方向に張り付く
        let mut want_x = 0;
        for j in 0..3 {
            if j != i {
                want_x += self.bx[j].div_ceil(res_y);
            }
        }
        if want_x <= self.x {
            return true;
        }

        // x 方向に張り付く
        let mut want_y = 0;
        for j in 0..3 {
            if j != i {
                want_y += self.bx[j].div_ceil(self.x);
            }
        }

        want_y <= res_y
    }

    fn can_y(&self, i: usize) -> bool {
        //! 箱 i を y 軸に張り付かせる
        if self.bx[i].div_ceil(self.y) >= self.x {
            return false;
        }

        let res_x = self.x - self.bx[i].div_ceil(self.y);
        // 使える領域は、 res_x, y の四角形

        // res_x 方向に張り付く
        let mut want_y = 0;
        for j in 0..3 {
            if j != i {
                want_y += self.bx[j].div_ceil(res_x);
            }
        }
        if want_y <= self.y {
            return true;
        }

        // y 方向に張り付く
        let mut want_x = 0;
        for j in 0..3 {
            if j != i {
                want_x += self.bx[j].div_ceil(self.y);
            }
        }

        want_x <= res_x
    }
}

fn main() {
    input! {
        (x, y, a, b, c): (usize, usize, usize, usize, usize),
    }

    let solve = Solve::new(x, y, a, b, c);

    for i in 0..3 {
        if solve.can_x(i) {
            println!("Yes");
            return;
        }
        if solve.can_y(i) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
