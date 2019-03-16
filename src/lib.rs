use std::cmp;

/*
/// a^3 + b^3 + c^3 = 33
/// (8866128975287528)^3 + (-8778405442862239)^3 + (-2736111468807040)^3 = 33
/// を検算するぜ☆（＾～＾）
fn main() {
    // 文字にするぜ☆（＾～＾）
    // マイナスは省くぜ☆（＾～＾）
    let a_text = "8866128975287528";
    let b_text = "8778405442862239";
    let c_text = "2736111468807040";

    // タイトル画面のような感じ☆（＾～＾）
    println!("\n({})^3 + (-{})^3 + (-{})^3 = 33",a_text, b_text, c_text);

    let a_num = to_digit_string(a_text);
    let b_num = to_digit_string(b_text);
    let c_num = to_digit_string(c_text);

    // インデックスを作る☆（＾～＾）どれも たまたま同じだが……☆（＾～＾）
    let aa_index = create_index(&a_num, &a_num);

    // a ☆（＾～＾）
    println!("\nLet's calculate a^3 ☆（＾～＾）");
    // a^2 を求める☆（＾～＾） 要するに九九だな☆（＾～＾）
    println!("\n({})^2 table.",a_text);
    let aa_kuku = multiplied_by(&a_num, &a_num);
    // ナナメの足し算だぜ☆（＾～＾）
    let a_sum = sum(&aa_index, &aa_kuku, a_text.len() * 2 - 1);
    let aa_text = accumulate(&a_sum);
    println!("aa_text {}\n", aa_text);
    // よっしゃ☆（＾～＾）！２週目だぜ☆（*＾～＾*）！
    // インデックスを作る☆（＾～＾）今度は長さが異なるぜ☆（＾～＾）
    let aa_num = to_digit_string(&aa_text);
    let aaa_index = create_index(&aa_num, &a_num);
    // さらに掛け算☆（*＾～＾*）
    println!("\n{} * {} table.",aa_text, a_text);
    let aaa_kuku = multiplied_by(&aa_num, &a_num);
    // ナナメの足し算だぜ☆（＾～＾）
    let a_sum = sum(&aaa_index, &aaa_kuku, aa_text.len() + a_text.len() - 1);
    let aaa_text = accumulate(&a_sum);
    println!("aaa_text  : {}\n", aaa_text);

    // b ☆（＾～＾）
    println!("\nLet's calculate b^3 ☆（＾～＾）");
    let bb_index = create_index(&b_num, &b_num);
    println!("\n({})^2 table.",b_text);
    let bb_kuku = multiplied_by(&b_num, &b_num);
    let b_sum = sum(&bb_index, &bb_kuku, b_text.len() * 2 - 1);
    let bb_text = accumulate(&b_sum);
    println!("bb_text {}\n", bb_text);
    let bb_num = to_digit_string(&bb_text);
    let bbb_index = create_index(&bb_num, &b_num);
    println!("\n{} * {} table.",bb_text, b_text);
    let bbb_kuku = multiplied_by(&bb_num, &b_num);
    let b_sum = sum(&bbb_index, &bbb_kuku, bb_text.len() + b_text.len() - 1);
    let bbb_text = accumulate(&b_sum);
    println!("bbb_text  : {}\n", bbb_text);

    // c ☆（＾～＾）
    println!("\nLet's calculate c^3 ☆（＾～＾）");
    let cc_index = create_index(&c_num, &c_num);
    println!("\n({})^2 table.",c_text);
    let cc_kuku = multiplied_by(&c_num, &c_num);
    let c_sum = sum(&cc_index, &cc_kuku, c_text.len() * 2 - 1);
    let cc_text = accumulate(&c_sum);
    println!("cc_text {}\n", cc_text);
    let cc_num = to_digit_string(&cc_text);
    let ccc_index = create_index(&cc_num, &c_num);
    println!("\n{} * {} table.",cc_text, c_text);
    let ccc_kuku = multiplied_by(&cc_num, &c_num);
    let c_sum = sum(&ccc_index, &ccc_kuku, cc_text.len() + c_text.len() - 1);
    let ccc_text = accumulate(&c_sum);
    println!("ccc_text  :  {}\n", ccc_text);

    // 引き算しようぜ☆（*＾～＾*）
    let aaa_num = to_digit_string(&aaa_text);
    let bbb_num = to_digit_string(&bbb_text);
    let ccc_num = to_digit_string(&ccc_text);

    // a - b - c ☆（＾～＾）
    println!("\nLet's a - b - c ☆（＾～＾）");
    println!("aaa_text  : {}", aaa_text);
    println!("bbb_text  : {}", bbb_text);
    let d_num = subtract(&aaa_num, &bbb_num);
    let d_text = to_string(&d_num);
    println!("a - b     = {} (d)", d_text);
    // println!("expected  =  20483367622797158223817952754905569383153664033");
    println!("ccc_text  :  {}", ccc_text);
    let e_num = subtract(&d_num, &ccc_num);
    let e_text = to_string(&e_num);
    println!("d - c     = {}", e_text);

    // a - c - b ☆（＾～＾）
    println!("\nLets's a - c - b ☆（＾～＾）");
    println!("aaa_text  : {}", aaa_text);
    println!("ccc_text  :  {}", ccc_text);
    let f_num = subtract(&aaa_num, &ccc_num);
    let f_text = to_string(&f_num);
    println!("a - c     = {} (f)", f_text);
    // println!("expected  = 676467453392982277424361019810585360331722557952");
    println!("bbb_text  : {}", bbb_text);
    let g_num = subtract(&f_num, &bbb_num);
    let g_text = to_string(&g_num);
    println!("f - b     = {}", g_text);
}
*/

// 桁がでかいので、数字列のまま引き算するぜ☆（＾～＾）
fn subtract(a_num:&Vec<i8>, b_num:&Vec<i8>) -> Vec<i8> {

    // 桁数がでかい方がえらい☆（＾～＾）
    let long_num;
    let short_num;
    if a_num.len() < b_num.len() {
        long_num = b_num;
        short_num = a_num;
    } else if a_num.len() > b_num.len() {
        long_num = a_num;
        short_num = b_num;
    } else {
        let len = a_num.len();
        if a_num[len-1] < b_num[len-1] {
            long_num = b_num;
            short_num = a_num;
        } else {
            long_num = a_num;
            short_num = b_num;
        }
    }
    /*
    let long_text = to_string(long_num);
    let short_text = to_string(short_num);
    println!("long_text  = {}", long_text);
    println!("short_text = {}", short_text);
     */

    let mut vec = Vec::new();

    // 下の桁から計算。
    let short_len = cmp::min(a_num.len(), b_num.len());
    let mut bollow = false;
    for column in 0..short_len {
        let pre_bollow = bollow;
        let mut long_n = if column < long_num.len() {long_num[column]} else {0};
        let short_n = if column < short_num.len() {short_num[column]} else {0};

        // 下の桁が、前借りしたせいで long_n様の数が 1 減ることになるとはな☆（＾～＾）
        let mut carry_payment = 0;
        if pre_bollow {
            carry_payment = -1;
        }

        // 各桁は 絶対値にして計算する。
        // 長い方から、短い方を引く。
        let mut carry_debt = 0;
        if (long_n.abs()+carry_payment) < short_n {
            // 引けなければ、上の桁から 1 を前借りして１０を足す☆（＾～＾）
            carry_debt = 10;
            bollow = true;
        } else {
            bollow = false;
        };

        let c = carry_payment + carry_debt + long_n.abs() - short_n.abs();
        println!("{} = {:2} + {:2} + {} - {}", c, carry_payment, carry_debt, long_n.abs(), short_n.abs());

        vec.push(c);
    }

    // 大きな桁の残ってる桁を最後に付けろだぜ☆（＾～＾）
    let long_len = cmp::max(a_num.len(), b_num.len());
    for column in short_len..long_len {
        let mut long_n = long_num[column];

        // 下の桁が、前借りしたせいで long_n様の数が 1 減ることになるとはな☆（＾～＾）
        if bollow {
            long_n -= 1;
        }

        if long_n < 0 {
            // まだ借りる☆（＾～＾）
            long_n += 10;
        } else {
            // チャラ☆（＾～＾）
            bollow = false;
        }

        println!("L {}", long_n);
        vec.push(long_n);
    }

    if bollow {
        // TODO ……☆（＾～＾）？
        println!("CARRY -1 ……☆（＾～＾）？");
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

// 掛け算☆（＾～＾）
fn multiplied_by(a_num:&Vec<i8>, b_num:&Vec<i8>) -> Vec<i8> {
    // 何やってるか見たいときはプリントしろだぜ☆（＾～＾）
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

// 数字の一列を、文字列にして返すぜ☆（*＾～＾*）
fn to_string(a_sum:&Vec<i8>) -> String {
    let mut number_text = "".to_string();

    for num in a_sum {
        number_text = format!("{}{}", num, number_text);
    }

    number_text
}

// 数字の一列にするぜ☆（＾～＾） 下の桁から配列に入れている☆（*＾～＾*）
fn to_digit_string(numbers:&str) -> Vec<i8> {
    let mut vec = Vec::new();
    for number_char in numbers.chars().rev() {
        let num: i8 = number_char.to_string().parse().unwrap();
        vec.push(num);
        // println!("{}", num);
    }
    vec
}