use std::cmp;

// 数だぜ☆（＾～＾）
pub struct HandyNumber {
    /// 偽ならマイナスの符号だぜ☆（＾ｑ＾）
    pub positive: bool,
    /// 下の桁から絶対値を配列に入れている☆（*＾～＾*）
    pub numbers: Vec<i8>,
}
impl HandyNumber {
    pub fn new() -> HandyNumber {
        HandyNumber {
            positive: true,
            numbers: Vec::new(),
        }
    }

    // 数字の一列にするぜ☆（＾～＾） 下の桁から配列に入れている☆（*＾～＾*）
    pub fn create(positive:bool, numbers:&str) -> HandyNumber {
        let mut vec = Vec::new();
        for number_char in numbers.chars().rev() {
            let num: i8 = number_char.to_string().parse().unwrap();
            vec.push(num);
            // println!("{}", num);
        }

        HandyNumber {
            positive: positive,
            numbers: vec,
        }
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    /// 1の位は 0 ☆（*＾～＾*）
    pub fn get_figure(&self, figure:usize) -> i8 {
        self.numbers[figure]
    }

    // 数字の一列を、文字列にして返すぜ☆（*＾～＾*） 符号は別にして返す☆（＾ｑ＾）
    fn to_string_parts(&self) -> (bool, String) {
        let mut number_text = "".to_string();

        for num in self.numbers.iter() {
            number_text = format!("{}{}", num, number_text);
        }

        (self.positive, number_text)
    }

    pub fn to_string(&self) -> String {
        let (positive, text) = self.to_string_parts();
        format!("{}{}", if positive {"".to_string()} else {"-".to_string()}, text)
    }

    // 掛け算☆（＾～＾）
    pub fn multiplied_by(&self, b_num:&HandyNumber) -> HandyNumber {
        // 何やってるか見たいときはプリントしろだぜ☆（＾～＾）
        print!("   ");
        for column in 0..b_num.len() {
            print!("{:>3}", b_num.get_figure(b_num.len()-column-1));
        }
        println!();

        let mut vec = Vec::new();
        for row in 0..self.len() {
            print!("{:>3}", self.get_figure(row));
            for column in 0..b_num.len() {
                let n = self.get_figure(row) * b_num.get_figure(b_num.len()-column-1);
                vec.push(n);
                print!("{:>3}", n);
            }
            println!();
        }
        println!();

        HandyNumber {
            positive: !(self.positive ^ self.positive),
            numbers: vec,
        }
    }

    // 桁がでかいので、数字列のまま引き算するぜ☆（＾～＾）
    pub fn subtract(&self, b_num:&HandyNumber) -> HandyNumber {

        // 桁数がでかい方がえらい☆（＾～＾）
        let mut swapping = false;
        let long_num;
        let short_num;
        if &self.len() < &b_num.len() {
            long_num = b_num;
            short_num = &self;
            swapping = true;
        } else if &self.len() > &b_num.len() {
            long_num = &self;
            short_num = &b_num;
        } else {
            let len = &self.len();
            if &self.get_figure(len-1) < &b_num.get_figure(len-1) {
                long_num = b_num;
                short_num = &self;
                swapping = true;
            } else {
                long_num = &self;
                short_num = &b_num;
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
        let short_len = cmp::min(self.len(), b_num.len());
        let mut bollow = false;
        for column in 0..short_len {
            let pre_bollow = bollow;
            let mut long_n = if column < long_num.len() {long_num.get_figure(column)} else {0};
            let short_n = if column < short_num.len() {short_num.get_figure(column)} else {0};

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
        let long_len = cmp::max(self.len(), b_num.len());
        for column in short_len..long_len {
            let mut long_n = long_num.get_figure(column);

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
        HandyNumber {
            positive: !swapping,
            numbers: vec,
        }
    }
}
