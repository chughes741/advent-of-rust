
fn part_one(counter: &mut i64, value: i64) {
    let string = value.to_string();

    if string.len() % 2 != 0 {
        return;
    }

    let (first, last) = string.split_at(string.len() / 2);

    if first == last {
        *counter += value;
    }
}

fn part_two(counter: &mut i64, value: i64) {
    let string = value.to_string();
    let len = string.len();

    for size in 1..=len / 2 {
        if len % size != 0 {
            continue;
        }

        let (substr, _) = string.split_at(size);
        let duplicate = substr.repeat(len / size);

        if string == duplicate {
            *counter += value;
            return;
        }
    }
}

fn main() {
    let contents: String = std::fs::read_to_string("input")
        .expect("Could not read input file.");

    let ranges: Vec<&str> = contents.trim().split(',').collect();

    let bounds: Vec<(i64, i64)> = ranges.iter()
        .map(|value| {
            let result: Vec<&str> = value.split('-').collect();
            (result[0].parse::<i64>().expect(""), result[1].parse::<i64>().expect(""))
        })
        .collect();

    let mut counter: i64 = 0;

    for bound in bounds.iter() {
        for value in bound.0..=bound.1 {
            part_two(&mut counter, value);
        }
    }

    println!("Counter: {}", counter);
}
