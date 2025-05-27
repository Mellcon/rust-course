/// Порахувати мінімальну кількість переносу грузу
/// Повертає кількість одиниць переносу
pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        panic!("Рівномірний розподіл неможливий!");
    }

    let average = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &load in shipments {
        balance += (load as i32 - average as i32);
        moves += balance.abs() as usize;
    }

    moves
}

/// Функція генерує вектор грузів, який можна рівномірно розподілити
pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![0; n];
    for i in 0..n {
        shipments[i] = 4; // приклад: однакове число
    }
    shipments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let v1 = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&v1), 4);

        let v2 = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&v2), 7);
    }
}

fn main() {
    let shipments = vec![8, 2, 2, 4, 4];
    println!("Shipments: {:?}", shipments);
    println!("Moves needed: {}", count_permutation(&shipments));

    let generated = gen_shipments(5);
    println!("Generated shipments: {:?}", generated);
}
