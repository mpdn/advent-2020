const ONE: &str = std::include_str!("1.txt");

fn one() {
    let expenses: Vec<u32> = ONE.lines()
        .map(|s| s.parse().unwrap())
        .collect();

    'outer_a:
    for a in &expenses {
        for b in &expenses {
            if a + b == 2020 {
                println!("{}", a * b);
                break 'outer_a;
            }
        }
    }

    'outer_b:
    for a in &expenses {
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

fn main() {
    one();  
}
