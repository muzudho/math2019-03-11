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
/// cargo run --example magic_table
/// ```
extern crate math2019_03_11;

use math2019_03_11::*;
use math2019_03_11::handy_number::*;

fn main() {
    // 外和用テーブル
    println!("\nExternul sum.");
    create_external_sum_table();

    // 内和用テーブル
    println!("\nInternul sum.");
    create_internal_sum_table();
}