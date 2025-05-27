[dependencies]
itertools = "0.12"

use itertools::Itertools;

pub fn solve_cryptarithm() {
    let digits = (1..=8).collect::<Vec<_>>();
    let mut count = 0;

    for perm in digits.iter().permutations(8).unique() {
        let m = *perm[0] as u32;
        let u = *perm[1] as u32;
        let x = *perm[2] as u32;
        let a = *perm[3] as u32;
        let s = *perm[4] as u32;
        let l = *perm[5] as u32;
        let o = *perm[6] as u32;
        let n = *perm[7] as u32;

        let muha = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muha * a == slon {
            count += 1;
            println!(
                "{}\nx   {}\n-----\n{:>5}\n",
                muha, a, slon
            );
        }
    }

    println!("Кількість рішень: {}", count);
}
