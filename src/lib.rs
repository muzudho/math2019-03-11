pub mod handy_number;

use handy_number::*;

// 桁がでかいので、文字列にして返すぜ☆（*＾～＾*）
pub fn accumulate(a_sum:&[i32]) -> String {
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

pub fn sum(a_index:&[i8], a_kuku:&HandyNumber, capacity:usize) -> Vec<i32> {
    let mut vec : Vec<i32> = Vec::with_capacity(capacity);
    for _i in 0..capacity {
        vec.push(0);
    }

    for (i, index) in a_index.iter().enumerate() {
    //for i in 0..a_index.len() {
        //let index = a_index[i] as usize;
        let n = vec[*index as usize] + i32::from(a_kuku.get_figure(i));
        vec[*index as usize] = n;
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

