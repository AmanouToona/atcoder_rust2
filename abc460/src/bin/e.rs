#![allow(non_snake_case)]
use ac_library::ModInt;
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
桁数で考えていくのが良さそう
y digit = 1..=(Nの桁数 高々 18 桁)
正整数なので 1 より大きい

もし制約なく1ずつ振るのであれば、 M 毎に条件を満たす
- 1桁増えた状態での最小値、最大値は取得できる
- 最大値については N でキャップされるので注意
- そのまま生成すると overflowする ... 桁が増えた状態での最小値が、 mod M のいくつかを把握する


*/
fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            (N, M): (usize, usize)
        }
        let mut ans = mint::new(0);

        ModInt::set_modulus(M as u32);
        let mut digit10: [i128; 20] = [1; 20];
        for i in 1..19 {
            digit10[i] = digit10[i - 1] * 10;
        }

        let digit = N.to_string().len();
        for i in 1..=digit {
            let mut min = ModInt::new(N) * digit10[i];
            if i != digit {
                // max までには、 digit10[i] この数字がある
                // M - min が最初の割り切れる数字
            }
        }

        // digit == 1 の時の y = 0 の処理
    }
}
