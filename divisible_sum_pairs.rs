use std::collections::HashMap;

fn divisible_sum_pairs(ar: &[i32], k: i32) -> i32 {
    let mut remainder_count = HashMap::new();
    let mut count = 0;
    for num in ar {
        let remainder = num % k;
        count += remainder_count.get(&(&(k - remainder) % k)).cloned().unwrap_or(0);
        *remainder_count.entry(remainder).or_insert(0) += 1;
    }
    count
}
fn main() {
    let ar = vec![1, 2, 3, 4, 5, 6];
    let k = 5;

    let result = divisible_sum_pairs(&ar, k);
    println!("Кількість пар: {}", result);
}