/// (8866128975287528)^3 + (-8778405442862239)^3 + (-2736111468807040)^3 = 33
/// を検算するぜ☆（＾～＾）
fn main() {
    // 文字にするぜ☆（＾～＾）
    let a_text = "8866128975287528";
    let b_text = "8778405442862239";
    let c_text = "2736111468807040";

    // マイナスは省くぜ☆（＾～＾）
    let a_num = divide(a_text);
    let b_num = divide(b_text);
    let c_num = divide(c_text);

    // インデックスを作る☆（＾～＾）どれも たまたま同じだが……☆（＾～＾）
    let a_index = create_index(&a_num);
    let b_index = create_index(&b_num);
    let c_index = create_index(&c_num);

    // a^2 を求める☆（＾～＾） 要するに九九だな☆（＾～＾）
    let a_kuku = power2(&a_num);
    let b_kuku = power2(&b_num);
    let c_kuku = power2(&c_num);

    // ナナメの足し算だぜ☆（＾～＾）
    let a_sum = sum(&a_index, &a_kuku, a_text.len() * 2 - 1);
    let b_sum = sum(&b_index, &b_kuku, b_text.len() * 2 - 1);
    let c_sum = sum(&c_index, &c_kuku, c_text.len() * 2 - 1);

    let a_a_text = accumulate(&a_sum);
    let b_b_text = accumulate(&b_sum);
    let c_c_text = accumulate(&c_sum);
    println!("a_a_text {}", a_a_text);
    println!("b_b_text {}", b_b_text);
    println!("c_c_text {}", c_c_text);
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

fn power2(a:&Vec<i8>) -> Vec<i8> {
    print!("   ");
    for column in 0..a.len() {
        print!("{:>3}", a[a.len()-column-1]);
    }
    println!();

    let mut vec = Vec::new();
    for row in 0..a.len() {
        print!("{:>3}", a[row]);
        for column in 0..a.len() {
            let n = a[row] * a[a.len()-column-1];
            vec.push(n);
            print!("{:>3}", n);
        }
        println!();
    }
    println!();
    vec
}

fn create_index(a:&Vec<i8>) -> Vec<i8> {
    print!("   ");
    for column in 0..a.len() {
        print!("{:>3}", a[a.len()-column-1]);
    }
    println!();

    let mut vec = Vec::new();
    for row in 0..a.len() {
        print!("{:>3}", a[row]);
        for column in 0..a.len() {
            let index : i8 = (row + (a.len() - column - 1)) as i8;
            vec.push(index);
            print!("{:>3}", index);
        }
        println!();
    }
    println!();
    vec
}

fn divide(numbers:&str) -> Vec<i8> {
    let mut vec = Vec::new();
    for number_char in numbers.chars().rev() {
        let num: i8 = number_char.to_string().parse().unwrap();
        vec.push(num);
        // println!("{}", num);
    }
    vec
}