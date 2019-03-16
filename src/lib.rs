pub mod handy_number;

use handy_number::*;

// 桁がでかいので、文字列にして返すぜ☆（*＾～＾*）
pub fn accumulate(a_sum:&Vec<i32>) -> String {
    let mut number_text = "".to_string();

    let mut cumulus = 0;
    for num in a_sum {
        cumulus += num;
        number_text = format!("{}{}", cumulus % 10, number_text);
        cumulus /= 10;
    }

    while cumulus > 0 {
        number_text = format!("{}{}", cumulus % 10, number_text);
        cumulus /= 10;
    }

    number_text
}

pub fn sum(a_index:&Vec<i8>, a_kuku:&HandyNumber, capacity:usize) -> Vec<i32> {
    let mut vec : Vec<i32> = Vec::with_capacity(capacity);
    for _i in 0..capacity {
        vec.push(0);
    }

    for i in 0..a_index.len() {
        let index = a_index[i] as usize;
        let n = vec[index] + a_kuku.get_figure(i) as i32;
        vec[index as usize] = n;
    }

    println!(" Sinister diagonal sum.");
    for column in 0..vec.len() {
        for _indent in 0..column {
            print!(" ");
        }
        print!("{:>3}", vec[vec.len()-column-1]);
        println!();
    }
    println!();

    vec
}

pub fn create_index(a_num:&HandyNumber, b_num:&HandyNumber) -> Vec<i8> {
    // 何やってるか見たいときはプリントしろだぜ☆（＾～＾）
    /*
    print!("   ");
    for column in 0..b_num.len() {
        print!("{:>3}", b_num[b_num.len()-column-1]);
    }
    println!();
     */

    let mut vec = Vec::new();
    for row in 0..a_num.len() {
        // print!("{:>3}", a_num[row]);
        for column in 0..b_num.len() {
            let index : i8 = (row + (b_num.len() - column - 1)) as i8;
            vec.push(index);
            // print!("{:>3}", index);
        }
        // println!();
    }
    // println!();
    vec
}

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

pub fn create_regular_row(y:i8) -> Vec<i8> {
    // 何やってるか見たいときはプリントしろだぜ☆（＾～＾）
    let mut axis = Vec::new();

    print!("{:>3}|", y);

    for i in 0..=8 {
        let x = -9 + i;
        axis.push((x+y)%10);
        print!("{:>3}", axis[axis.len()-1]);
    }
    
    for x in 0..=9 {
        axis.push((x+y)%10);
        print!("{:>3}", axis[axis.len()-1]);
    }

    println!();
    axis
}
/// 外和用。
pub fn create_regular_table() {
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

    for y in 0..=9 {
        create_regular_row(9-y);
    }
    for y in 1..=9 {
        create_regular_row(-y);
    }
}
