use std::io;

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][n - i - 1];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні рядка");
    let n: usize = input.trim().parse().expect("Помилка при перетворенні в число");

    let mut matrix = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Помилка при читанні рядка");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Помилка при перетворенні елемента в число"))
            .collect();
        matrix.push(row);
    }

    let result = diagonal_difference(&matrix);
    println!("{}", result);
}