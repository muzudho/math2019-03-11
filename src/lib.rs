use std::cmp;

// 桁がでかいので、数字列のまま引き算するぜ☆（＾～＾）
pub fn subtract(a_num:&Vec<i8>, b_num:&Vec<i8>) -> (bool, Vec<i8>) {

    // 桁数がでかい方がえらい☆（＾～＾）
    let mut swapping = false;
    let long_num;
    let short_num;
    if a_num.len() < b_num.len() {
        long_num = b_num;
        short_num = a_num;
        swapping = true;
    } else if a_num.len() > b_num.len() {
        long_num = a_num;
        short_num = b_num;
    } else {
        let len = a_num.len();
        if a_num[len-1] < b_num[len-1] {
            long_num = b_num;
            short_num = a_num;
            swapping = true;
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

    // スワッピングしてたら、符号はマイナスだぜ☆（＾～＾）
    (!swapping, vec)
}

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

pub fn sum(a_index:&Vec<i8>, a_kuku:&Vec<i8>, capacity:usize) -> Vec<i32> {
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
pub fn multiplied_by(a_num:&Vec<i8>, b_num:&Vec<i8>) -> Vec<i8> {
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

pub fn create_index(a_num:&Vec<i8>, b_num:&Vec<i8>) -> Vec<i8> {
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

// 数字の一列を、文字列にして返すぜ☆（*＾～＾*） 符号は別にして返す☆（＾ｑ＾）
pub fn to_string(positive_sign:bool, a_sum:&Vec<i8>) -> (bool, String) {
    let mut number_text = "".to_string();

    for num in a_sum {
        number_text = format!("{}{}", num, number_text);
    }

    (positive_sign, number_text)
}

// 数字の一列にするぜ☆（＾～＾） 下の桁から配列に入れている☆（*＾～＾*）
pub fn to_digit_string(numbers:&str) -> Vec<i8> {
    let mut vec = Vec::new();
    for number_char in numbers.chars().rev() {
        let num: i8 = number_char.to_string().parse().unwrap();
        vec.push(num);
        // println!("{}", num);
    }
    vec
}

// 偽ならマイナスの符号を返すぜ☆（＾ｑ＾）
pub fn to_sign_string(positive_sign:bool) -> String {
    if positive_sign {"".to_string()} else {"-".to_string()}
}