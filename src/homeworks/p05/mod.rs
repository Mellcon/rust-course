// Функція для обчислення НОД за алгоритмом Евкліда
pub fn find_gcd(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        let remainder = x % y;
        x = y;
        y = remainder;
    }
    x
}

// Тестова функція
fn main() {
    let num1 = 36;
    let num2 = 60;
    println!("НОД чисел {} і {} дорівнює {}", num1, num2, find_gcd(num1, num2));
}
