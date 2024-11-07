use std::io;

fn plus_minus(arr: &[i32]) {
    let n = arr.len() as f64;
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    let positive_ratio = positive_count as f64 / n;
    let negative_ratio = negative_count as f64 / n;
    let zero_ratio = zero_count as f64 / n;

    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні рядка");
    let n: usize = input.trim().parse().expect("Помилка при перетворенні в число");

    input.clear();
    io::stdin().read_line(&mut input).expect("Помилка при читанні рядка");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Помилка при перетворенні елемента в число"))
        .collect();

    if arr.len() != n {
        eprintln!("Помилка: кількість чисел у масиві не збігається з вказаним значенням n");
        return;
    }

    plus_minus(&arr);
}
