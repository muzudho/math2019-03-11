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

use math2019_03_11::*;

/// Test a subtract.
fn main()
{
    // 文字にするぜ☆（＾～＾）
    // マイナスは省くぜ☆（＾～＾）
    let a_text = "637";
    let b_text = "1492";

    // タイトル画面のような感じ☆（＾～＾）
    println!("\n{} - {}",a_text, b_text);

    let a_num = to_digit_string(a_text);
    let b_num = to_digit_string(b_text);

    // a - b ☆（＾～＾）
    println!("\nLet's a - b ☆（＾～＾）");
    let (positive_sign, c_num) = subtract(&a_num, &b_num);
    let (positive_sign, c_text) = to_string(positive_sign, &c_num);
    println!("a - b     = {}{} (c)", to_sign_string(positive_sign) ,c_text);

}