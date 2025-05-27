/// Функція для циклічного зсуву рядка
pub fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    // Нормалізація зміщення
    let shift = ((n % len as isize) + len as isize) % len as isize;
    let shift = shift as usize;

    let mut result = String::with_capacity(len);
    result.push_str(&s[len - shift..]);
    result.push_str(&s[..len - shift]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)| 
                assert_eq!(
                    rotate(s.clone(), *n), 
                    exp.to_string()
                )
            );
    }
}
