//! score_circuit/src/lib.rs
//!
//! ゲームの最終スコアと敵撃破数からスコア計算を行い、正当性を検証する回路です。
//! 入力:
//!   公開入力: 最終スコア (u32)
//!   プライベート入力: 敵撃破数（タイプ 0～3、それぞれ u32）
//!
//! 計算:
//!   computed_score = count0 * 100 + count1 * 100 + count2 * 10000 + count3 * 10
//!
//! 検証:
//!   computed_score と最終スコアが一致するかチェック

#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // 公開入力として最終スコアを読み込む
    let final_score = sp1_zkvm::io::read::<u32>();

    // プライベート入力として、敵撃破数（タイプ 0～3）を読み込む
    let count0 = sp1_zkvm::io::read::<u32>();
    let count1 = sp1_zkvm::io::read::<u32>();
    let count2 = sp1_zkvm::io::read::<u32>();
    let count3 = sp1_zkvm::io::read::<u32>();

    // スコア計算
    let computed_score = count0 * 100 + count1 * 100 + count2 * 10000 + count3 * 10;

    // 計算結果と公開入力が一致しているか確認
    if final_score != computed_score {
        panic!(
            "Score mismatch: final_score {} != computed {}",
            final_score, computed_score
        );
    }

    // 結果をコミット（公開入力として証明に含める）
    sp1_zkvm::io::commit(&final_score);
    sp1_zkvm::io::commit(&computed_score);
}
