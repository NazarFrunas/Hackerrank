use std::io;

fn a_very_big_sum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні рядка");
    let n: usize = input.trim().parse().expect("Помилка при перетворенні в число");

    input.clear();
    io::stdin().read_line(&mut input).expect("Помилка при читанні рядка");
    let ar: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Помилка при перетворенні елемента в число"))
        .collect();

    if ar.len() != n {
        eprintln!("Помилка: кількість чисел у масиві не збігається з вказаним значенням n");
        return;
    }

    let result = a_very_big_sum(&ar);
    println!("{}", result);
}

