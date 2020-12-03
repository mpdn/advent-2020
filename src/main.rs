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

fn three() {
    let trees: Vec<Vec<_>> = std::include_str!("3.txt")
        .lines()
        .map(|s| {
            s.chars()
                .map(|ch| match ch {
                    '#' => true,
                    '.' => false,
                    ch => panic!("Unexpected char: {}", ch),
                })
                .collect()
        })
        .collect();

    let hits = |slope_x: usize, slope_y: usize| {
        let mut x = 0;
        let mut y = 0;
        let mut trees_hit: u64 = 0;

        while y < trees.len() {
            let line = &trees[y];
            if line[x % line.len()] {
                trees_hit += 1
            }

            x += slope_x;
            y += slope_y;
        }

        trees_hit
    };

    println!("{}", hits(3, 1));
    println!(
        "{}",
        hits(1, 1) * hits(3, 1) * hits(5, 1) * hits(7, 1) * hits(1, 2)
    );
}

fn main() {
    one();
    two();
    three();
}
