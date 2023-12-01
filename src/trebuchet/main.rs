use std::fs::read_to_string;

fn main() {
    let sum: i32 = read_to_string("src/trebuchet/test-case.txt")
        .unwrap()
        .lines()
        .map(parse)
        .fold(0, |acc, n| acc + n);

    println!("{sum}");
}

fn parse(s: &str) -> i32 {
    let mut first: Option<char> = None;
    let mut i: usize = 0;
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        if c >= '0' && c <= '9' {
            first = Some(c);
            break;
        }

        i += 1;
    }

    let first = match first {
        Some(n) => n,
        None => '0',
    };

    i += 1;

    let second: char = {
        let mut current: Option<char> = None;
        while i < s.len() {
            let c = s.chars().nth(i).unwrap();
            if c >= '0' && c <= '9' {
                current = Some(c);
            }

            i += 1;
        }

        match current {
            Some(n) => n,
            None => first,
        }
    };

    String::from_iter(vec![first, second]).parse().unwrap()
}
