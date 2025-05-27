use rand::Rng;

/// Створює вектор з випадковими числами в межах [10..99]
pub fn generate_random_array(count: usize) -> Vec<i32> {
    let mut generator = rand::thread_rng();
    (0..count).map(|_| generator.gen_range(10..100)).collect()
}

/// Повертає індекси пари сусідніх елементів з найменшою сумою
pub fn find_min_pair_indices(arr: &[i32]) -> (usize, usize) {
    arr.windows(2)
        .enumerate()
        .min_by_key(|(_, pair)| pair[0] + pair[1])
        .map(|(idx, _)| (idx, idx + 1))
        .unwrap()
}

/// Виводить вектор з позначенням пари з найменшою сумою
pub fn show_array_with_min_pair(arr: &[i32]) {
    let (a, b) = find_min_pair_indices(arr);

    // Вивід індексів
    let index_line = (0..arr.len())
        .map(|i| format!("{:2}.", i))
        .collect::<Vec<_>>()
        .join(" ");
    println!("indexes: {}", index_line);

    // Вивід елементів
    let data_line = arr.iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<_>>()
        .join(", ");
    println!("data:    [{}]", data_line);

    // Підсвітка мінімальної пари
    let pointer_line = format!("indexes: {}\\__ __/", " ".repeat(4 * a));
    println!("{}", pointer_line);

    // Пояснення
    let sum = arr[a] + arr[b];
    println!(
        "min adjacent sum = {} + {} = {} at indices {}, {}",
        arr[a], arr[b], sum, a, b
    );
}

fn main() {
    let vec = generate_random_array(20);
    show_array_with_min_pair(&vec);
}
