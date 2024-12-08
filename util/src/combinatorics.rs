pub fn pair_combinations<T>(arr: &Vec<T>) -> Vec<(&T, &T)> {
    let mut result = Vec::new();
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            result.push((&arr[i], &arr[j]));
        }
    }
    result
}
