use std::fs;

fn is_invalid(num: i64) -> bool {
    let num_str: String = num.to_string();

    if num_str.len() % 2 != 0 {
        return false;
    }

    let mid = num_str.len() / 2;
    let (start, end) = num_str.split_at(mid);

    return start == end;
}

fn is_invalid_p2(num: i64) -> bool {
    let num_str: String = num.to_string();

    for i in 1..=num_str.len() / 2 {
        if num_str.len() % i != 0 {
            continue;
        }

        let (start, mut remainder) = num_str.split_at(i);
        let mut next: &str;
        for j in 1..(num_str.len() / i) {
            (next, remainder) = remainder.split_at(i);

            if start != next {
                break;
            }

            if start == next && j == (num_str.len() / i) - 1 {
                return true;
            }
        }
    }

    return false;
}

fn get_total_of_range(start: i64, end: i64) -> i64 {
    let mut total = 0;
    for i in start..=end {
        if is_invalid(i) {
            total += i;
        }
    }

    return total;
}

fn get_total_of_range_p2(start: i64, end: i64) -> i64 {
    let mut total = 0;
    for i in start..=end {
        if is_invalid_p2(i) {
            total += i;
        }
    }

    return total;
}

fn main() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let ranges: Vec<&str> = input_str.split(",").collect();

    let mut total = 0;
    let mut total_p2 = 0;
    for range in ranges {
        let arr: Vec<&str> = range.split("-").collect();
        total += get_total_of_range(arr[0].parse().unwrap(), arr[1].parse().unwrap());
        total_p2 += get_total_of_range_p2(arr[0].parse().unwrap(), arr[1].parse().unwrap());
    }

    println!("Total Part 1: {}", total);
    println!("Total Part 2: {}", total_p2);
}
