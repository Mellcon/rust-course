/// Перевіряє, чи число просте.
/// Повертає true, якщо просте, інакше false.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Тестування функції
fn main() {
    let numbers = [2, 3, 4, 5, 10, 13, 15, 17, 19, 20, 23];
    
    for &n in &numbers {
        println!("{} is prime: {}", n, is_prime(n));
    }
}
