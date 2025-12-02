use std::fs;

fn main() {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut position: i32 = 50;

    let mut count = 0;

    for line in &lines {
        let (direction, amount) = line.split_at(1);

        let mut multiplier = 1;
        if direction.chars().nth(0).unwrap() == 'L' {
            multiplier = -1;
        }

        let amount = amount.parse::<i32>().unwrap() * multiplier;

        position += amount;

        let mut num_turns = (position as f32 / 100 as f32).floor().abs() as i32;

        // Don't double count if we hit zero straight on
        if position.rem_euclid(100) == 0 && position > 0 {
            num_turns -= 1;
        }

        // Don't double count when we move on after hitting zero straight on
        if position - amount == 0 && amount < 0 {
            num_turns -= 1;
        }

        position = position.rem_euclid(100);

        if position == 0 {
            count += 1;
        }
        count += num_turns;
    }

    println!("Count is {}", count);
}
