fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max = candles.iter().max().unwrap();
    candles.iter().filter(|&&c| c == *max).count() as i32
}