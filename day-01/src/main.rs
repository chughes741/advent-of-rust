

fn parse(instruction: &str) -> i32 {
    let (direction, magnitude) = instruction.split_at(1);

    let sign = if direction == "R" { 1 } else { -1 };
    let number = magnitude.parse::<i32>().expect("Could not parse to integer.");

    return sign * number;
}

fn main() {
    let contents: String = std::fs::read_to_string("input")
        .expect("Could not read input file.");

    let instructions: Vec<&str> = contents.split('\n').collect();

    let mut index = 0;
    let mut zero_count = 0;

    for instruction in instructions.iter() {
        if instruction.is_empty() {
            break;
        }

        index += parse(instruction);
        index %= 100;
        zero_count += if index == 0 { 1 } else { 0 };
    }
    println!("Landed on 0 {} times.", zero_count);
}

