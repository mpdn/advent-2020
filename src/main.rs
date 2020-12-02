use scan_fmt::scan_fmt;

fn one() {
    let expenses: Vec<u32> = std::include_str!("1.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    'outer_a: for a in &expenses {
        for b in &expenses {
            if a + b == 2020 {
                println!("{}", a * b);
                break 'outer_a;
            }
        }
    }

    'outer_b: for a in &expenses {
        for b in &expenses {
            for c in &expenses {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    break 'outer_b;
                }
            }
        }
    }
}

fn two() {
    let passwords: Vec<_> = std::include_str!("2.txt")
        .lines()
        .map(|s| scan_fmt!(s, "{d}-{d} {[a-z]}: {}", usize, usize, char, String).unwrap())
        .collect();

    let n_valid_a = passwords
        .iter()
        .filter(|&&(min, max, ch, ref pass)| {
            (min..=max).contains(&pass.chars().filter(|&c| c == ch).count())
        })
        .count();

    println!("{}", n_valid_a);

    let n_valid_b = passwords
        .iter()
        .filter(|&&(a, b, ch, ref pass)| {
            pass.chars()
                .enumerate()
                .filter(|&(i, c)| (i == a - 1 || i == b - 1) && c == ch)
                .count()
                == 1
        })
        .count();

    println!("{}", n_valid_b);
}

fn main() {
    one();
    two();
}
