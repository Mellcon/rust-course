const HEIGHT: usize = 11; // Висота ромба (має бути непарною!)

fn main() {
  let mut result = String::new();
  let mid = HEIGHT / 2;

  for i in 0..HEIGHT {
      let level = if i <= mid { i } else { HEIGHT - i - 1 };
      let spaces = mid - level;
      let stars = 2 * level + 1;

      result.push_str(&" ".repeat(spaces));
      result.push_str(&"*".repeat(stars));
      result.push('\n');
  }

  print!("{}", result);
}
