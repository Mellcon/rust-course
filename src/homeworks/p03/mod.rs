const WIDTH: usize = 40;  // Ширина конверта (від 10 до 80)
const HEIGHT: usize = 20; // Висота конверта (від 10 до 80)

fn main() {
    let mut canvas = String::new();

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let is_border_top_or_bottom = row == 0 || row == HEIGHT - 1;
            let is_border_sides = col == 0 || col == WIDTH - 1;
            let is_diagonal_main = col * (HEIGHT - 1) == row * (WIDTH - 1);
            let is_diagonal_second = col * (HEIGHT - 1) == (HEIGHT - 1 - row) * (WIDTH - 1);

            if is_border_top_or_bottom || is_border_sides || is_diagonal_main || is_diagonal_second {
                canvas.push('*');
            } else {
                canvas.push(' ');
            }
        }
        canvas.push('\n');
    }

    print!("{}", canvas); // Виводимо увесь малюнок за один раз
}
