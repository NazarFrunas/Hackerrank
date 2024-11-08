fn mini_max_sum(arr: &[i32])
{
    let mut sorted = arr.to_vec();
    sorted.sort();
    let min_sum = sorted.iter().take(sorted.len() - 1).map(|&x| x as i64).sum::<i64>();
    let max_sum = sorted.iter().skip(1).map(|&x| x as i64).sum::<i64>();
    println!("{} {}", min_sum, max_sum);
}
#[test]
fn test()
{
    mini_max_sum(&[396285104, 573261094, 759641832, 819230764, 364801279]);
}