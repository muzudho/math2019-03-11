/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cd C:\muzudho\projects_rust\math2019-03-11
/// cargo clippy --example main
/// 
/// ### 実行。
/// cls
/// cargo run --example main
/// ```

extern crate math2019_03_11;

use math2019_03_11::*;
use math2019_03_11::handy_number::*;

/// a^3 + b^3 + c^3 = 33
/// (8866128975287528)^3 + (-8778405442862239)^3 + (-2736111468807040)^3 = 33
/// を検算するぜ☆（＾～＾）
fn main()
{
    // 文字にするぜ☆（＾～＾）
    // マイナスは省くぜ☆（＾～＾）
    let a_text = "8866128975287528";
    let b_text = "8778405442862239";
    let c_text = "2736111468807040";

    // タイトル画面のような感じ☆（＾～＾）
    println!("\n({})^3 + (-{})^3 + (-{})^3 = 33",a_text, b_text, c_text);

    let a_num = HandyNumber::create(true, a_text);
    let b_num = HandyNumber::create(false, b_text);
    let c_num = HandyNumber::create(false, c_text);

    // インデックスを作る☆（＾～＾）どれも たまたま同じだが……☆（＾～＾）
    let aa_index = create_index(&a_num, &a_num);

    // a ☆（＾～＾）
    println!("\nLet's calculate a^3 ☆（＾～＾）");
    // a^2 を求める☆（＾～＾） 要するに九九だな☆（＾～＾）
    println!("\n({})^2 table.",a_text);
    let aa_kuku = a_num.multiplied_by(&a_num);
    // ナナメの足し算だぜ☆（＾～＾）
    let a_sum = sum(&aa_index, &aa_kuku, a_text.len() * 2 - 1);
    let aa_text = accumulate(&a_sum);
    println!("aa_text {}\n", aa_text);
    // よっしゃ☆（＾～＾）！２週目だぜ☆（*＾～＾*）！
    // インデックスを作る☆（＾～＾）今度は長さが異なるぜ☆（＾～＾）
    let aa_num = HandyNumber::create(aa_kuku.positive, &aa_text);
    let aaa_index = create_index(&aa_num, &a_num);
    // さらに掛け算☆（*＾～＾*）
    println!("\n{} * {} table.",aa_text, a_text);
    let aaa_kuku = aa_num.multiplied_by(&a_num);
    // ナナメの足し算だぜ☆（＾～＾）
    let a_sum = sum(&aaa_index, &aaa_kuku, aa_text.len() + a_text.len() - 1);
    let aaa_text = accumulate(&a_sum);
    println!("aaa_text  : {}\n", aaa_text);

    // b ☆（＾～＾）
    println!("\nLet's calculate b^3 ☆（＾～＾）");
    let bb_index = create_index(&b_num, &b_num);
    println!("\n({})^2 table.",b_text);
    let bb_kuku = &b_num.multiplied_by(&b_num);
    let b_sum = sum(&bb_index, &bb_kuku, b_text.len() * 2 - 1);
    let bb_text = accumulate(&b_sum);
    println!("bb_text {}\n", bb_text);
    let bb_num = HandyNumber::create(bb_kuku.positive, &bb_text);
    let bbb_index = create_index(&bb_num, &b_num);
    println!("\n{} * {} table.",bb_text, b_text);
    let bbb_kuku = &bb_num.multiplied_by(&b_num);
    let b_sum = sum(&bbb_index, &bbb_kuku, bb_text.len() + b_text.len() - 1);
    let bbb_text = accumulate(&b_sum);
    println!("bbb_text  : {}\n", bbb_text);

    // c ☆（＾～＾）
    println!("\nLet's calculate c^3 ☆（＾～＾）");
    let cc_index = create_index(&c_num, &c_num);
    println!("\n({})^2 table.",c_text);
    let cc_kuku = &c_num.multiplied_by(&c_num);
    let c_sum = sum(&cc_index, &cc_kuku, c_text.len() * 2 - 1);
    let cc_text = accumulate(&c_sum);
    println!("cc_text {}\n", cc_text);
    let cc_num = HandyNumber::create(cc_kuku.positive, &cc_text);
    let ccc_index = create_index(&cc_num, &c_num);
    println!("\n{} * {} table.",cc_text, c_text);
    let ccc_kuku = &cc_num.multiplied_by(&c_num);
    let c_sum = sum(&ccc_index, &ccc_kuku, cc_text.len() + c_text.len() - 1);
    let ccc_text = accumulate(&c_sum);
    println!("ccc_text  :  {}\n", ccc_text);

    // 引き算しようぜ☆（*＾～＾*）
    let aaa_num = HandyNumber::create(aaa_kuku.positive, &aaa_text);
    let bbb_num = HandyNumber::create(bbb_kuku.positive, &bbb_text);
    let ccc_num = HandyNumber::create(ccc_kuku.positive, &ccc_text);

    // a - b - c ☆（＾～＾）
    println!("\nLet's a - b - c ☆（＾～＾）");
    println!("aaa_text  : {}", aaa_text);
    println!("bbb_text  : {}", bbb_text);
    let d_num = aaa_num.subtract(&bbb_num);
    println!("a - b     = {} (d)", d_num.to_string());
    // println!("expected  =  20483367622797158223817952754905569383153664033");
    println!("ccc_text  :  {}", ccc_text);
    let e_num = d_num.subtract(&ccc_num);
    println!("d - c     = {}", e_num.to_string());

    // a - c - b ☆（＾～＾）
    println!("\nLets's a - c - b ☆（＾～＾）");
    println!("aaa_text  : {}", aaa_text);
    println!("ccc_text  :  {}", ccc_text);
    let f_num = aaa_num.subtract(&ccc_num);
    println!("a - c     = {} (f)", f_num.to_string());
    // println!("expected  = 676467453392982277424361019810585360331722557952");
    println!("bbb_text  : {}", bbb_text);
    let g_num = f_num.subtract(&bbb_num);
    println!("f - b     = {}", g_num.to_string());
}