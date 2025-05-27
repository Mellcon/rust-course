pub fn print_christmas_tree(tiers: usize) {
    let width = tiers * 2 + 1;

    for tier in 0..tiers {
        for line in 0..=(tier + 1) {
            let branch_count = 2 * line + 1;
            let offset = width - line - 1;
            let branch_line = format!("{}{}", " ".repeat(offset), "*".repeat(branch_count));
            println!("{}", branch_line);
        }
    }

    // Ствол
    let stem_width = 1;
    let stem_height = 2;
    let stem_indent = width - 1;

    for _ in 0..stem_height {
        println!("{}{}", " ".repeat(stem_indent), "|");
    }
}

fn main() {
    let tree_tiers = 3;
    print_christmas_tree(tree_tiers);
}
