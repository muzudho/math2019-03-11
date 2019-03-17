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
    {
        println!("\n(0) Let's |a|≦|b|, 0≦a, 0≦b ☆（＾～＾）");

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

    {
        println!("\n(1) Let's |a|≦|b|, 0≦a, b＜0 ☆（＾～＾）");

        let a_num = HandyNumber::create(true, "637");
        let b_num = HandyNumber::create(false, "1492");

        // タイトル画面のような感じ☆（＾～＾）
        println!("\n{} + {}", a_num.to_string(), b_num.to_string());

        // 筆算の形に☆（＾～＾）
        println!("\n  {:>10}", a_num.to_string());
        println!("+ {:>10}", b_num.to_string());
        println!("--------------");

        let c_num = a_num.add(&b_num);
        println!("a + b     = {} (c)", c_num.to_string());
        println!("Expected    -855.");
    }

    {
        println!("\n(2) Let's |a|≦|b|, a＜0, b＜0 ☆（＾～＾）");

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

    {
        println!("\n(3) Let's |a|≦|b|, a＜0, 0≦b ☆（＾～＾）");

        let a_num = HandyNumber::create(false, "637");
        let b_num = HandyNumber::create(true, "1492");

        // タイトル画面のような感じ☆（＾～＾）
        println!("\n{} + {}", a_num.to_string(), b_num.to_string());

        // 筆算の形に☆（＾～＾）
        println!("\n  {:>10}", a_num.to_string());
        println!("+ {:>10}", b_num.to_string());
        println!("--------------");

        let c_num = a_num.add(&b_num);
        println!("a + b     = {} (c)", c_num.to_string());
        println!("Expected     855.");
    }

    {
        println!("\nLet's |a|≧|b|, 0≦a, b＜0 ☆（＾～＾）");

        let a_num = HandyNumber::create(true, "1492");
        let b_num = HandyNumber::create(false, "637");

        // タイトル画面のような感じ☆（＾～＾）
        println!("\n{} + {}", a_num.to_string(), b_num.to_string());

        // 筆算の形に☆（＾～＾）
        println!("\n  {:>10}", a_num.to_string());
        println!("+ {:>10}", b_num.to_string());
        println!("--------------");

        let c_num = a_num.add(&b_num);
        println!("a + b     = {} (c)", c_num.to_string());
        println!("Expected     855.");
    }
}