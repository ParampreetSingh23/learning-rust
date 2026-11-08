fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_nums: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * 10)
        .collect();
       println!("{:?}", new_nums);
}