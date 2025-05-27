/// Обертає рядок циклічно на задану кількість позицій
pub fn shift_string(input: String, offset: isize) -> String {
    let length = input.len();
    if length == 0 {
        return input;
    }

    // Обчислюємо фактичний зсув
    let actual_shift = ((offset % length as isize + length as isize) % length as isize) as usize;

    let mut output = String::with_capacity(length);
    output.push_str(&input[length - actual_shift..]);
    output.push_str(&input[..length - actual_shift]);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifting_works() {
        let base = "abcdefgh".to_string();
        let cases = vec![
            (0, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (10, "ghabcdef"),
            (-10, "cdefghab"),
        ];

        for (offset, expected) in cases {
            let result = shift_string(base.clone(), offset);
            assert_eq!(result, expected);
        }
    }
}
