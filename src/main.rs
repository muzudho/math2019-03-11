/// (8866128975287528)^3 + (-8778405442862239)^3 + (-2736111468807040)^3 = 33
/// を検算するぜ☆（＾～＾）
fn main() {
    // マイナスは省くぜ☆（＾～＾）
    let a = divide("8866128975287528");
    let b = divide("8778405442862239");
    let c = divide("2736111468807040");

    // まず a^3 を求める☆（＾～＾）
    power2(a);
    power2(b);
    power2(c);
}

fn power2(a:Vec<i8>) {
    print!("   ");
    for column in 0..a.len() {
        print!("{:>3}", a[a.len()-column-1]);
    }
    println!();

    /*
    for row in 0..a.len() {
        print!("{:>3}", a[row]);
        for column in 0..a.len() {
            print!("{:>3}", a[row] * a[a.len()-column-1]);    
        }
        println!();
    }
     */
    for row in 0..a.len() {
        print!("{:>3}", a[row]);
        for column in 0..a.len() {
            print!("{:>3}", row + (a.len() - column - 1));
        }
        println!();
    }
    println!();
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