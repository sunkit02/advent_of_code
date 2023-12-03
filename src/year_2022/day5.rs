pub fn solve_part1(input: &str) -> String {
    let crane = CrateMover9000;
    return part1(input, &crane);
}

fn part1(input: &str, crane: &impl Crane) -> String {
    let mut top_crates = Vec::new();
    let (cargo_stacks, instructions) = input.split_once("\n\n").expect("Wrong input format");

    let mut cargo_stacks = parse_cargo_stacks(cargo_stacks);
    let instructions = parse_instructions(instructions);

    crane.move_crates(&mut cargo_stacks, &instructions);

    for mut stack in cargo_stacks {
        if let Some(top) = stack.pop() {
            top_crates.push(top as u8);
        }
    }

    return String::from_utf8(top_crates).unwrap_or("".to_owned());
}

pub fn solve_part2(input: &str) -> String {
    let crane = CrateMover9001;
    return part2(input, &crane);
}

fn part2(input: &str, crane: &impl Crane) -> String {
    let mut top_crates = Vec::new();
    let (cargo_stacks, instructions) = input.split_once("\n\n").expect("Wrong input format");

    let mut cargo_stacks = parse_cargo_stacks(cargo_stacks);
    let instructions = parse_instructions(instructions);

    crane.move_crates(&mut cargo_stacks, &instructions);

    for mut stack in cargo_stacks {
        if let Some(top) = stack.pop() {
            top_crates.push(top as u8);
        }
    }

    return String::from_utf8(top_crates).unwrap_or("".to_owned());
}

trait Crane {
    fn move_crates(&self, cargo_stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>);
}

struct CrateMover9000;

impl Crane for CrateMover9000 {
    fn move_crates(&self, cargo_stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) {
        for inst in instructions {
            for _ in 0..inst.n {
                let c = cargo_stacks[inst.from].pop().expect("Stack is empty");
                cargo_stacks[inst.to].push(c);
            }
        }
    }
}

struct CrateMover9001;

impl Crane for CrateMover9001 {
    fn move_crates(&self, cargo_stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) {
        let mut crane_arm = Vec::new();
        for inst in instructions {
            // Load crates into crane arm from src stack
            for _ in 0..inst.n {
                let c = cargo_stacks[inst.from].pop().expect("Stack is empty");
                crane_arm.push(c);
            }

            // Move crates from crane arm to target stack
            for c in crane_arm.drain(0..crane_arm.len()).rev() {
                cargo_stacks[inst.to].push(c);
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    /// Index of src stack
    from: usize,
    /// Index of target stack
    to: usize,
    /// Number of crates to move
    n: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut splits = value.split(" ");
        splits.next();
        let n = match splits.next() {
            Some(n) => n.parse::<usize>().map_err(|e| e.to_string())?,
            None => return Err(format!("No move num.")),
        };

        splits.next();

        let from = if let Some(n) = splits.next() {
            // Subtract 1 for index;
            n.parse::<usize>().map_err(|e| e.to_string())? - 1
        } else {
            return Err(format!("No from num."));
        };

        splits.next();

        let to = if let Some(n) = splits.next() {
            // Subtract 1 for index;
            n.parse::<usize>().map_err(|e| e.to_string())? - 1
        } else {
            return Err(format!("No from num."));
        };

        Ok(Self { from, to, n })
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Move {} from {} to {}",
            self.n,
            self.from + 1,
            self.to + 1
        ))
    }
}

fn parse_cargo_stacks(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let line_len = lines.get(0).expect("No cargo stacks").len();
    let num_stacks = line_len / 4 + 1;

    let mut stacks = vec![vec![]; num_stacks];
    stacks.fill(Vec::default());

    for (_y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_uppercase() {
                // Indices of crate values: 1 5 9 13 17 21 25 29 33
                // or all x's where (x - 1) % 4 == 0
                if (x - 1) % 4 == 0 {
                    stacks[(x - 1) / 4].insert(0, c);
                }
            }
        }
    }

    return stacks;
}
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|line| Instruction::try_from(line).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        assert_eq!(solve_part1(sample_input), "CMZ".to_owned())
    }

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        assert_eq!(solve_part2(sample_input), "MCD".to_owned())
    }
}
