pub fn draw_tree(levels: usize) {
    // function
    let total_width = levels * 2 + 1;

    for level in 0..levels {
        for row in 0..=level + 1 {
            let stars = 2 * row + 1;
            let spaces = total_width - row - 1;
            let line = " ".repeat(spaces) + &"*".repeat(stars);
            println!("{}", line);
        }
    }

    // draw
    let trunk_width = 1;
    let trunk_height = 2;
    let trunk_padding = total_width - 1;

    for _ in 0..trunk_height {
        println!("{}{}", " ".repeat(trunk_padding), "|");
    }
}

// test
fn main() {
    let levels = 3;
    draw_tree(levels);
}
