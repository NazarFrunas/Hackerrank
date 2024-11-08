use std::collections::HashMap;

fn sock_merchant(ar: &[i32]) -> i32 {
    let mut sock_counts = HashMap::new();

    for &color in ar {
        *sock_counts.entry(color).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for count in sock_counts.values() {
        pairs += count / 2;
    }

    pairs
}

fn main() {
    let ar = vec![1, 2, 1, 2, 1, 3, 2];
    let result = sock_merchant(&ar);
    println!("Кількість пар носків: {}", result);
}