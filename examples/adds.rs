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
    // 文字列で絶対値を指定するぜ☆（＾～＾）
    let a_num = HandyNumber::create(true, "637");
    let b_num = HandyNumber::create(true, "1492");

    // タイトル画面のような感じ☆（＾～＾）
    println!("\n{} + {}", a_num.to_string(), b_num.to_string());

    // 筆算の形に☆（＾～＾）
    println!("\n  {:>10}", a_num.to_string());
    println!("+ {:>10}", b_num.to_string());
    println!("--------------");

    // a + b ☆（＾～＾）
    println!("\nLet's a + b ☆（＾～＾）");
    let c_num = a_num.add(&b_num);
    println!("a + b     = {} (c)", c_num.to_string());
}