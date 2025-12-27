

fn parse(instruction: &str) -> (i32, i32) {
    let (direction, distance) = instruction.split_at(1);

    let sign = if direction == "R" { 1 } else { -1 };
    let number = distance.parse::<i32>().expect("Could not parse to integer.");

    ((number / 100), sign * (number % 100))
}

fn main() {
    let contents: String = std::fs::read_to_string("input")
        .expect("Could not read input file.");

    let instructions: Vec<&str> = contents.split('\n').collect();

    let mut index = 50;
    let mut count = 0;

    for instruction in instructions.iter() {
        if instruction.is_empty() {
            break;
        }

        let (turns, rotation) = parse(instruction);

        count += turns;
        index += rotation;
        count += if index >= 100 || index < 0 { 1 } else { 0 };
        index = index.abs() % 100;
    }
    println!("Passed zero {} times.", count);
}

