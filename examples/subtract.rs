/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cd C:\muzudho\projects_rust\math2019-03-11
/// cargo clippy --example subtract
/// 
/// ### 実行。
/// cls
/// cargo run --example subtract
/// ```
extern crate math2019_03_11;

use math2019_03_11::handy_number::*;

/// Test a subtract.
fn main()
{
    // 文字にするぜ☆（＾～＾）
    // マイナスは省くぜ☆（＾～＾）
    let a_text = "637";
    let b_text = "1492";

    // タイトル画面のような感じ☆（＾～＾）
    println!("\n{} - {}",a_text, b_text);

    let a_num = HandyNumber::create(true, a_text);
    let b_num = HandyNumber::create(true, b_text);

    // a - b ☆（＾～＾）
    println!("\nLet's a - b ☆（＾～＾）");
    let c_num = a_num.subtract(&b_num);
    println!("a - b     = {} (c)", c_num.to_string());

}