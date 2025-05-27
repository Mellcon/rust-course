use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Rectangle {
    a: Point,
    b: Point,
}

/// Генеруємо тестові дані
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

/// Обчислення зайнятої площі
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied_points: HashSet<(i32, i32)> = HashSet::new();

    for rect in xs {
        let (left, right) = (rect.a.x.min(rect.b.x), rect.a.x.max(rect.b.x));
        let (bottom, top) = (rect.a.y.min(rect.b.y), rect.a.y.max(rect.b.y));

        for x in left..right {
            for y in bottom..top {
                occupied_points.insert((x, y));
            }
        }
    }

    occupied_points.len() as i32
}

/// Тест
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}

fn main() {
    let data = test_data();
    let area = area_occupied(&data);
    println!("Occupied area: {}", area);
}
