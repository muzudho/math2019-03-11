use std::cmp;

/// (8866128975287528)^3 + (-8778405442862239)^3 + (-2736111468807040)^3 = 33
/// を検算するぜ☆（＾～＾）
fn main() {
    // 文字にするぜ☆（＾～＾）
    let a_text = "8866128975287528";
    let b_text = "8778405442862239";
    let c_text = "2736111468807040";

    // マイナスは省くぜ☆（＾～＾）
    let a_num = to_digit_string(a_text);
    let b_num = to_digit_string(b_text);
    let c_num = to_digit_string(c_text);

    // インデックスを作る☆（＾～＾）どれも たまたま同じだが……☆（＾～＾）
    let aa_index = create_index(&a_num, &a_num);
    let bb_index = create_index(&b_num, &b_num);
    let cc_index = create_index(&c_num, &c_num);

    // a^2 を求める☆（＾～＾） 要するに九九だな☆（＾～＾）
    let aa_kuku = multiplied_by(&a_num, &a_num);
    let bb_kuku = multiplied_by(&b_num, &b_num);
    let cc_kuku = multiplied_by(&c_num, &c_num);

    // ナナメの足し算だぜ☆（＾～＾）
    let a_sum = sum(&aa_index, &aa_kuku, a_text.len() * 2 - 1);
    let b_sum = sum(&bb_index, &bb_kuku, b_text.len() * 2 - 1);
    let c_sum = sum(&cc_index, &cc_kuku, c_text.len() * 2 - 1);

    let aa_text = accumulate(&a_sum);
    let bb_text = accumulate(&b_sum);
    let cc_text = accumulate(&c_sum);
    println!("aa_text {}", aa_text);
    println!("bb_text {}", bb_text);
    println!("cc_text {}", cc_text);

    // よっしゃ☆（＾～＾）！２週目だぜ☆（*＾～＾*）！
    let aa_num = to_digit_string(&aa_text);
    let bb_num = to_digit_string(&bb_text);
    let cc_num = to_digit_string(&cc_text);

    // インデックスを作る☆（＾～＾）今度は長さが異なるぜ☆（＾～＾）
    let aaa_index = create_index(&aa_num, &a_num);
    let bbb_index = create_index(&bb_num, &b_num);
    let ccc_index = create_index(&cc_num, &c_num);

    // さらに掛け算☆（*＾～＾*）
    let aaa_kuku = multiplied_by(&aa_num, &a_num);
    let bbb_kuku = multiplied_by(&bb_num, &b_num);
    let ccc_kuku = multiplied_by(&cc_num, &c_num);

    // ナナメの足し算だぜ☆（＾～＾）
    let a_sum = sum(&aaa_index, &aaa_kuku, aa_text.len() + a_text.len() - 1);
    let b_sum = sum(&bbb_index, &bbb_kuku, bb_text.len() + b_text.len() - 1);
    let c_sum = sum(&ccc_index, &ccc_kuku, cc_text.len() + c_text.len() - 1);

    let aaa_text = accumulate(&a_sum);
    let bbb_text = accumulate(&b_sum);
    let ccc_text = accumulate(&c_sum);
    println!("aaa_text {}", aaa_text);
    println!("bbb_text {}", bbb_text);
    println!("ccc_text {}", ccc_text);

    // 引き算しようぜ☆（*＾～＾*）
    let aaa_num = to_digit_string(&aaa_text);
    let bbb_num = to_digit_string(&bbb_text);
    let ccc_num = to_digit_string(&ccc_text);
    let result = subtract(&aaa_num, &bbb_num);
    let result_text = to_string(&result);
    println!("result {}", result_text);
}

// 桁がでかいので、数字列のまま引き算するぜ☆（＾～＾）
fn subtract(a_num:&Vec<i8>, b_num:&Vec<i8>) -> Vec<i8> {
    let len = cmp::max(a_num.len(), b_num.len());
    let mut vec = Vec::new();

    let mut cumulus = 0;
    for column in 0..len{
        // 下の桁から計算。
        let index = len - column;
        let a = if index < a_num.len() {a_num[index]} else {0};
        let b = if index < b_num.len() {10-b_num[index]} else {10};

        let c = if b<a { (a+b-cumulus)%10} else {cumulus+=1; b};
        vec.push(c);

        if b<a {cumulus = 0;}
    }
    vec
}

// 桁がでかいので、文字列にして返すぜ☆（*＾～＾*）
fn accumulate(a_sum:&Vec<i32>) -> String {
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

fn sum(a_index:&Vec<i8>, a_kuku:&Vec<i8>, capacity:usize) -> Vec<i32> {
    let mut vec : Vec<i32> = Vec::with_capacity(capacity);
    for _i in 0..capacity {
        vec.push(0);
    }

    for i in 0..a_index.len() {
        let index = a_index[i] as usize;
        let n = vec[index] + a_kuku[i] as i32;
        vec[index as usize] = n;
    }

    println!(" Naname sum.");
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

// 掛け算☆（＾～＾）
fn multiplied_by(a_num:&Vec<i8>, b_num:&Vec<i8>) -> Vec<i8> {
    print!("   ");
    for column in 0..b_num.len() {
        print!("{:>3}", b_num[b_num.len()-column-1]);
    }
    println!();

    let mut vec = Vec::new();
    for row in 0..a_num.len() {
        print!("{:>3}", a_num[row]);
        for column in 0..b_num.len() {
            let n = a_num[row] * b_num[b_num.len()-column-1];
            vec.push(n);
            print!("{:>3}", n);
        }
        println!();
    }
    println!();
    vec
}

fn create_index(a_num:&Vec<i8>, b_num:&Vec<i8>) -> Vec<i8> {
    print!("   ");
    for column in 0..b_num.len() {
        print!("{:>3}", b_num[b_num.len()-column-1]);
    }
    println!();

    let mut vec = Vec::new();
    for row in 0..a_num.len() {
        print!("{:>3}", a_num[row]);
        for column in 0..b_num.len() {
            let index : i8 = (row + (b_num.len() - column - 1)) as i8;
            vec.push(index);
            print!("{:>3}", index);
        }
        println!();
    }
    println!();
    vec
}

// 数字の一列を、文字列にして返すぜ☆（*＾～＾*）
fn to_string(a_sum:&Vec<i8>) -> String {
    let mut number_text = "".to_string();

    for num in a_sum {
        number_text = format!("{}{}", num, number_text);
    }

    number_text
}

// 数字の一列にするぜ☆（＾～＾）
fn to_digit_string(numbers:&str) -> Vec<i8> {
    let mut vec = Vec::new();
    for number_char in numbers.chars().rev() {
        let num: i8 = number_char.to_string().parse().unwrap();
        vec.push(num);
        // println!("{}", num);
    }
    vec
}