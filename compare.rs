fn compare_triplets(a: [i32; 3], b: [i32; 3]) -> (i32, i32) {
    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    (alice_score, bob_score)
}

fn main() {
    let alice = [5, 6, 7];
    let bob = [3, 6, 10];  

    let (alice_score, bob_score) = compare_triplets(alice, bob);

    println!("Аліса отримала {} балів", alice_score);
    println!("Боб отримав {} балів", bob_score);
}