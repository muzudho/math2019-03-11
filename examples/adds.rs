/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cd C:\muzudho\projects_rust\math2019-03-11
/// cargo clippy --example adds
/// 
/// ### 実行。
/// cls
/// cargo run --example adds
/// ```
extern crate math2019_03_11;

use math2019_03_11::handy_number::*;

/// Test a adds.
fn main()
{
    // 正＋正のケース☆（＾～＾）
    {
        // a + b ☆（＾～＾）
        println!("\nLet's positive a + positive b ☆（＾～＾）");

        // 文字列で絶対値を指定するぜ☆（＾～＾）
        let a_num = HandyNumber::create(true, "637");
        let b_num = HandyNumber::create(true, "1492");

        // タイトル画面のような感じ☆（＾～＾）
        println!("\n{} + {}", a_num.to_string(), b_num.to_string());

        // 筆算の形に☆（＾～＾）
        println!("\n  {:>10}", a_num.to_string());
        println!("+ {:>10}", b_num.to_string());
        println!("--------------");

        let c_num = a_num.add(&b_num);
        println!("a + b     = {} (c)", c_num.to_string());
        println!("Expected    2129.");
    }

    // 負＋負のケース☆（＾～＾）
    {
        // a + b ☆（＾～＾）
        println!("\nLet's negative a + negative b ☆（＾～＾）");

        let a_num = HandyNumber::create(false, "637");
        let b_num = HandyNumber::create(false, "1492");

        // タイトル画面のような感じ☆（＾～＾）
        println!("\n{} + {}", a_num.to_string(), b_num.to_string());

        // 筆算の形に☆（＾～＾）
        println!("\n  {:>10}", a_num.to_string());
        println!("+ {:>10}", b_num.to_string());
        println!("--------------");

        let c_num = a_num.add(&b_num);
        println!("a + b     = {} (c)", c_num.to_string());
        println!("Expected    -2129.");
    }
}