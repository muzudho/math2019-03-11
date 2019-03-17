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

    // キャリーが見やすいテーブル
    println!("\nCarry table.");
    create_carry_table();
}

/*
pub fn create_zero_row() -> Vec<i8> {
    let mut axis = Vec::new();

    for x in 1..=21 {
        axis.push(0);
        print!("{:>3}", axis[axis.len()-1]);
    }

    println!();
    axis
}
pub fn create_none_row() -> Vec<i8> {
    let mut axis = Vec::new();

    for x in 1..=21 {
        axis.push(0);
        print!("---");
    }

    println!();
    axis
}
pub fn create_magic_top_half_row(y:i8) -> Vec<i8> {
    // 何やってるか見たいときはプリントしろだぜ☆（＾～＾）
    let mut axis = Vec::new();

    print!("{:>3}|", y);

    for i in 0..9 {
        // 第２象限は補数にする。
        let nega_x = i - 9;
        let num = (nega_x+y)%10;
        let com = (10 - num)%10;
        axis.push(com);
        print!("{:>3}", axis[axis.len()-1]);
    }

    for x in 0..=9 {
        axis.push((x+y)%10);
        print!("{:>3}", axis[axis.len()-1]);
    }

    println!();
    axis
}
pub fn create_magic_bottom_half_row(nega_y:i8) -> Vec<i8> {
    // 何やってるか見たいときはプリントしろだぜ☆（＾～＾）
    let mut axis = Vec::new();

    print!("{:>3}|", nega_y);

    for i in 0..9 {
        let nega_x = i - 9;
        let com_x = 10 + nega_x;
        axis.push((nega_x+nega_y)%10);
        print!("{:>3}", axis[axis.len()-1]);
    }

    for x in 0..=9 {
        // 第４象限は補数にする。
        let num = (x+nega_y)%10;
        let com = (10 - num)%10;
        axis.push(com);
        print!("{:>3}", axis[axis.len()-1]);
    }

    println!();
    axis
}
// 内和用。
pub fn create_magic_table() {
    print!("   |");
    for x in -9..=0 {
        print!("{:>3}", x);
    }
    for x in 1..=9 {
        print!("{:>3}", x);
    }
    println!();

    print!("   +");
    for x in -9..=9 {
        print!(" --");
    }
    println!();

    for y in 1..=10 {
        create_magic_top_half_row(10-y);
    }
    for y in 1..=9 {
        create_magic_bottom_half_row(-y);
    }
}
*/

pub fn create_internal_sum_top_half_row(y:i8) -> Vec<i8> {
    let mut axis = Vec::new();

    print!("{:>3}|", y);

    // 第二象限は補数。
    for i in 0..=9 {
        let nega_x = -9 + i;
        let num = (nega_x+y)%10;
        let com = (10 - num.abs())%10*-1;
        axis.push(com);
        print!("{:>3}", axis[axis.len()-1]);
    }
    
    // 第一象限は無い。
    for x in 0..=9 {
        axis.push(0);
        print!("   ");
    }

    println!();
    axis
}
pub fn create_internal_sum_center_row(y:i8) -> Vec<i8> {
    let mut axis = Vec::new();

    print!("{:>3}|", y);
    for x in -9..=0 {
        let num = (x as i8).abs();
        let com = (10 - num)%10*-1;
        axis.push(com);
        print!("{:>3}", com);
    }
    for x in 1..=9 {
        let num = (x as i8).abs();
        let com = (10 - num)%10*-1;
        axis.push(com);
        print!("{:>3}", com);
    }
    println!();

    axis
}
pub fn create_internal_sum_bottom_half_row(nega_y:i8) -> Vec<i8> {
    let mut axis = Vec::new();

    print!("{:>3}|", nega_y);

    // 第三象限は無い。
    for i in 0..=8 {
        axis.push(0);
        print!("   ");
    }
    
    // 第４象限は補数。
    for x in 0..=9 {
        let num = (x + nega_y)%10;
        let com = (10 - num.abs())%10*-1;
        axis.push(com);
        print!("{:>3}", axis[axis.len()-1]);
    }

    println!();
    axis
}
// 内和用。
pub fn create_internal_sum_table() {
    print!("   |");
    for x in -9..=0 {
        print!("{:>3}", x);
    }
    for x in 1..=9 {
        print!("{:>3}", x);
    }
    println!();

    print!("   +");
    for x in -9..=9 {
        print!(" --");
    }
    println!();

    for y in 0..=8 {
        create_internal_sum_top_half_row(9-y);
    }
    create_internal_sum_center_row(0);
    for y in 1..=9 {
        create_internal_sum_bottom_half_row(-y);
    }
}

pub fn create_external_sum_top_half_row(y:i8) -> Vec<i8> {
    let mut axis = Vec::new();

    print!("{:>3}|", y);

    // 第二象限は無い。
    for i in 0..=8 {
        axis.push(0);
        print!("   ");
    }

    // 第四象限。
    for x in 0..=9 {
        axis.push((x+y)%10);
        print!("{:>3}", axis[axis.len()-1]);
    }

    println!();
    axis
}
pub fn create_external_sum_center_row(y:i8) -> Vec<i8> {
    let mut axis = Vec::new();

    print!("{:>3}|", y);
    for x in -9..=0 {
        axis.push(x);
        print!("{:>3}", x);
    }
    for x in 1..=9 {
        axis.push(x);
        print!("{:>3}", x);
    }
    println!();

    axis
}
pub fn create_external_sum_bottom_half_row(y:i8) -> Vec<i8> {
    // 何やってるか見たいときはプリントしろだぜ☆（＾～＾）
    let mut axis = Vec::new();

    print!("{:>3}|", y);

    // 第三象限。
    for i in 0..=9 {
        let x = -9 + i;
        let num = (x+y)%10;
        axis.push(num);
        print!("{:>3}", axis[axis.len()-1]);
    }

    // 第四象限は無い。 
    for x in 0..=9 {
        axis.push(0);
        print!("   ");
    }

    println!();
    axis
}
/// 外和用。
pub fn create_external_sum_table() {
    print!("   |");
    for x in -9..=0 {
        print!("{:>3}", x);
    }
    for x in 1..=9 {
        print!("{:>3}", x);
    }
    println!();

    print!("   +");
    for x in -9..=9 {
        print!(" --");
    }
    println!();

    for y in 0..=8 {
        create_external_sum_top_half_row(9-y);
    }
    create_external_sum_center_row(0);
    for y in 1..=9 {
        create_external_sum_bottom_half_row(-y);
    }
}

pub fn get_carry_type(num:i8) -> i8 {
    if num.abs() < 6 {
        0
    } else if num.abs() < 10 {
        6
    } else {
        10
    }
}
pub fn create_carry_top_half_row(y:i8) -> Vec<i8> {
    let mut axis = Vec::new();

    print!("{:>3}|", y);

    // 第二象限。
    for i in 0..=8 {
        let x = -9 + i;
        let num = get_carry_type(x + y);
        axis.push(num);
        print!("{:>3}", axis[axis.len()-1]);
    }

    // 第四象限。
    for x in 0..=9 {
        let num = get_carry_type(x + y);
        axis.push(num);
        print!("{:>3}", axis[axis.len()-1]);
    }

    println!();
    axis
}
pub fn create_carry_center_row(y:i8) -> Vec<i8> {
    let mut axis = Vec::new();

    print!("{:>3}|", y);
    for x in -9..=0 {
        let num = get_carry_type(x);
        axis.push(num);
        print!("{:>3}", axis[axis.len()-1]);
    }
    for x in 1..=9 {
        let num = get_carry_type(x);
        axis.push(num);
        print!("{:>3}", axis[axis.len()-1]);
    }
    println!();

    axis
}
pub fn create_carry_bottom_half_row(y:i8) -> Vec<i8> {
    // 何やってるか見たいときはプリントしろだぜ☆（＾～＾）
    let mut axis = Vec::new();

    print!("{:>3}|", y);

    // 第三象限。
    for i in 0..=8 {
        let x = -9 + i;
        let num = get_carry_type(x + y);
        axis.push(num);
        print!("{:>3}", axis[axis.len()-1]);
    }

    // 第四象限。 
    for x in 0..=9 {
        let num = get_carry_type(x + y);
        axis.push(num);
        print!("{:>3}", axis[axis.len()-1]);
    }

    println!();
    axis
}
/// 繰り上がりが見やすいテーブル
pub fn create_carry_table() {
    print!("   |");
    for x in -9..=0 {
        print!("{:>3}", x);
    }
    for x in 1..=9 {
        print!("{:>3}", x);
    }
    println!();

    print!("   +");
    for x in -9..=9 {
        print!(" --");
    }
    println!();

    for y in 0..=8 {
        create_carry_top_half_row(9-y);
    }
    create_carry_center_row(0);
    for y in 1..=9 {
        create_carry_bottom_half_row(-y);
    }
}
