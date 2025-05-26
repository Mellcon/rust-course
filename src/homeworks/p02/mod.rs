const W: usize = 40; // Ширина конверта (від 10 до 80)
const H: usize = 20; // Висота конверта (від 10 до 80)

fn main() {
    let mut result = String::new();

    for y in 0..H {
        for x in 0..W {
            // межі рамки
            if y == 0 || y == H - 1 {
                result.push('*');
            } else if x == 0 || x == W - 1 {
                result.push('*');
            }
            // діагоналі
            else if x * (H - 1) == y * (W - 1) || x * (H - 1) == (H - 1 - y) * (W - 1) {
                result.push('*');
            }
            // порожні місця
            else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    // Один раз виводимо весь результат
    print!("{}", result);
}
