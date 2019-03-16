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

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    /// 1の位は 0 ☆（*＾～＾*）
    pub fn get_figure(&self, figure:usize) -> i8 {
        self.numbers[figure]
    }
}
