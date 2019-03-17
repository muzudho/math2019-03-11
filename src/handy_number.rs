use std::cmp;

// 数だぜ☆（＾～＾）
pub struct HandyNumber {
    /// 偽ならマイナスの符号だぜ☆（＾ｑ＾）
    pub positive: bool,
    /// 下の桁から絶対値を配列に入れている☆（*＾～＾*）
    pub numbers: Vec<i8>,
}
impl HandyNumber {
    pub fn default() -> HandyNumber {
        HandyNumber {
            positive: true,
            numbers: Vec::new(),
        }
    }

    pub fn clone(source:&HandyNumber) -> HandyNumber {
        HandyNumber {
            positive: source.positive,
            numbers: source.numbers.clone(),
        }
    }

    // 数字の一列にするぜ☆（＾～＾） 下の桁から配列に入れている☆（*＾～＾*）
    pub fn create(positive_sign:bool, numbers:&str) -> HandyNumber {
        let mut vec = Vec::new();
        for number_char in numbers.chars().rev() {
            let num: i8 = number_char.to_string().parse().unwrap();
            vec.push(num);
            // println!("{}", num);
        }

        HandyNumber {
            positive: positive_sign,
            numbers: vec,
        }
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.numbers.len() < 1
    }

    /// 1の位は 0 ☆（*＾～＾*）
    pub fn get_figure(&self, figure:usize) -> i8 {
        self.numbers[figure]
    }

    pub fn set_figure(&mut self, figure:usize, value:i8) {
        self.numbers[figure] = value;
    }

    pub fn add_figure(&mut self, figure:usize, value:i8) {
        self.numbers[figure] += value;
    }

    /// 数字の一列を、文字列にして返すぜ☆（*＾～＾*） 符号は別にして返す☆（＾ｑ＾）
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

        // positive * positive = positive
        // positive * negative = negative
        // negative * positive = negative
        // negative * negative = positive
        HandyNumber {
            positive: self.positive == b_num.positive,
            numbers: vec,
        }
    }

    // 桁がでかいので、数字列のまま引き算するぜ☆（＾～＾）
    pub fn subtract(&self, b_num:&HandyNumber) -> HandyNumber {

        // 桁数がでかい方がえらい☆（＾～＾）
        let mut swapping = false;
        let long_num;
        let short_num;

        if self.len() < b_num.len() {
            long_num = b_num;
            short_num = &self;
            swapping = true;
        } else if self.len() > b_num.len() {
            long_num = &self;
            short_num = &b_num;
        } else {
            let len = &self.len();
            if self.get_figure(len-1) < b_num.get_figure(len-1) {
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
            };

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

    /// 足し算のルーチンだぜ☆（＾～＾）
    fn add_routine2(long_num:&mut HandyNumber, short_num:&mut HandyNumber, use_complement:bool) -> Vec<i8> {
        let mut vec = Vec::new();

        /*
        let long_text = to_string(long_num);
        let short_text = to_string(short_num);
        println!("long_text  = {}", long_text);
        println!("short_text = {}", short_text);
        */

        // 補数を使う場合。
        if use_complement {
            // 2桁以上の数だけ、キャリーダウンが使える。
            if 1 < long_num.numbers.len() {
                // 左項をキャリーダウン。
                for i in 0..long_num.numbers.len() {
                    if i == 0 {
                        // 一の位に変化なし。
                    } else if i == long_num.numbers.len() - 1 {
                        // 最上位は１減る。
                        long_num.add_figure(i, -1);
                    } else {
                        // 中間は9増える。
                        long_num.add_figure(i, 9);
                    }
                }

                // 右項は補数にする。
                for i in 0..short_num.numbers.len() {
                    let complement = 10 - short_num.get_figure(i);
                    short_num.set_figure(i, complement);
                }
            }
        };

        // 下の桁から計算。
        let short_len = cmp::min(long_num.len(), short_num.len());
        let mut carry_up = false;
        for column in 0..short_len {
            let pre_carry_up = carry_up;
            let mut long_n = if column < long_num.len() {long_num.get_figure(column)} else {0};
            let short_n = if column < short_num.len() {short_num.get_figure(column)} else {0};

            // 下の桁が、繰り上げしたから long_n様の数が 1 増えるぜ☆（＾～＾）
            let mut carry_value = 0;
            if pre_carry_up {
                carry_value = 1;
            };

            let b = carry_value + (long_n.abs()%10) + (short_n.abs()%10);
            if 9 < b {
                carry_up = true;
            } else {
                carry_up = false;
            }
            let c = (b)%10;

            println!("{:1}, {:>2} = {:>2} + {:>2} + {:>2}", if carry_up {1}else{0}, c, carry_value, long_n.abs(), short_n.abs());


            vec.push(c);
        }

        // 大きな桁の残ってる桁を最後に付けろだぜ☆（＾～＾）
        let long_len = cmp::max(long_num.len(), short_num.len());
        for column in short_len..long_len {
            let pre_carry_up = carry_up;
            let mut long_n = long_num.get_figure(column);

            // 下の桁が、繰り上げしたから long_n様の数が 1 増えるぜ☆（＾～＾）
            let mut carry_value = 0;
            if pre_carry_up {
                carry_value = 1;
            };

            let b = carry_value + (long_n%10);
            if 9 < b {
                // また繰り上げ☆（＾～＾）
            } else {
                // チャラ☆（＾～＾）
                carry_up = false;
            }
            let c = (b)%10;
            println!("{:1}, {:>2} = {:>2} + {:>2}", if carry_up {1}else{0}, c, carry_value, long_n);
            vec.push(c);

        }

        if carry_up {
            // TODO ……☆（＾～＾）？
            println!("CARRY UP……☆（＾～＾）？");
        }

        vec
    }

    /// 足し算のルーチンだぜ☆（＾～＾）
    fn add_routine1(a_number:&HandyNumber, b_number:&HandyNumber) -> HandyNumber {
        let mut a_num = HandyNumber::clone(a_number);
        let mut b_num = HandyNumber::clone(b_number);

        // 左項、右項がともに負の場合は、左右反転させようぜ☆（＾～＾）
        let flip_horizontal = if !a_num.positive && !b_num.positive {
            a_num.positive = true;
            b_num.positive = true;
            println!("Flip horizontal.");
            true
        } else {
            false
        };

        // 左項、右項の符号が異なれば、補数を使うぜ☆（＾～＾）
        let (use_complement, absolute_flip_horizontal) = if a_num.positive != b_num.positive {
            let absolute_flip_horizontal = if a_num.len() < b_num.len() && !b_num.positive {
                // 右項の方が桁数が長く、かつ右項が負なら、 absolute flip horizontal する。
                true
            } else if b_num.len() < a_num.len() && !a_num.positive {
                // 左項の方が桁数が長く、かつ左項が負なら、 absolute flip horizontal する。
                true
            } else {
                false
            };

            (true, absolute_flip_horizontal)
        } else {
            (false, false)
        };

        if absolute_flip_horizontal {
            // 符号を 全とっかえ させる。
            println!("Absolute flip horizontal.");
            a_num.positive = !a_num.positive;
            b_num.positive = !b_num.positive;
        }

        println!("a.positive: {}, b.positive: {}.", a_num.positive, b_num.positive);
        if use_complement {
            println!("Use complement.");
        }

        // 桁数がでかい方が左項☆（＾～＾）
        let vec = if a_num.len() < b_num.len() {
            println!("Swap sequence.");
            HandyNumber::add_routine2(&mut b_num, &mut a_num, use_complement)
        } else if b_num.len() < a_num.len() {
            HandyNumber::add_routine2(&mut a_num, &mut b_num, use_complement)
        } else {
            // 長さが同じなら、最上位桁の数で比較。
            let len = a_num.len();
            if a_num.get_figure(len-1) < b_num.get_figure(len-1) {
                println!("Swap sequence.");
                HandyNumber::add_routine2(&mut b_num, &mut a_num, use_complement)
            } else {
                HandyNumber::add_routine2(&mut a_num, &mut b_num, use_complement)
            }
        };

        // 左右反転させてたら、元に戻そうぜ☆（＾～＾）
        let mut result_positive_sign = if flip_horizontal {
            println!("Reverse flip horizontal.");
            false
        } else {
            true
        };

        // 絶対値で左右反転させてたら、結果の符号を反転させるぜ☆（*＾～＾*）
        if absolute_flip_horizontal {
            print!("Reverse absolute flip horizontal: {} --> ", result_positive_sign);
            result_positive_sign = !result_positive_sign;
            println!("{}.", result_positive_sign);
        };

        HandyNumber {
            positive: result_positive_sign,
            numbers: vec,
        }
    }

    // 足し算☆（＾～＾）
    // 正＋正　に対応☆（＾～＾）
    pub fn add(&self, b_num:&HandyNumber) -> HandyNumber {
        HandyNumber::add_routine1(&self, b_num)
    }
}
