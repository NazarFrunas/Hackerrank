fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let n = s.len();
    let mut suffix_sums = vec![0; n + 1];
    for i in (0..n).rev() {
        suffix_sums[i] = suffix_sums[i + 1] + s[i];
    }

    let mut count = 0;
    for i in 0..(n - m + 1) {
        let segment_sum = suffix_sums[i] - suffix_sums[i + m];
        if segment_sum == d {
            count += 1;
        }
    }

    count
}

fn main() {
    let s = vec![2, 2, 1, 3, 2];
    let d = 4;
    let m = 2;

    let result = birthday(&s, d, m);
    println!("Кількість способів: {}", result);
}