const SIZE: usize = 11; // Висота ромба (має бути непарною)

fn main() {
    let mut output = String::new();
    let center = SIZE / 2;

    for line in 0..SIZE {
        let distance = center as isize - (center as isize - line as isize).abs();
        let padding = center - distance as usize;
        let symbol_count = 2 * distance as usize + 1;

        output += &" ".repeat(padding);
        output += &"*".repeat(symbol_count);
        output.push('\n');
    }

    print!("{}", output);
}
