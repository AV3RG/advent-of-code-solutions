use iter_tools::Itertools;

advent_of_code::solution!(23);

#[derive(Debug)]
struct Machine {
    a: i64,
    b: i64,
    pointer: i64
}

impl Machine {
    fn new() -> Machine {
        Machine { a: 0, b: 0, pointer: 0 }
    }

    fn execute_instruction(&mut self, instruction: &str) {
        let (instruction, rest) = instruction.split_once(" ").unwrap();
        match instruction {
            "inc" => { if rest == "a" { self.a += 1; } else if rest == "b" { self.b += 1; } },
            "tpl" => { if rest == "a" { self.a *= 3; } else if rest == "b" { self.b *= 3;} },
            "hlf" => { if rest == "a" { self.a /= 2; } else if rest == "b" { self.b /= 2;} },
            "jmp" => {
                self.pointer += rest.parse::<i64>().unwrap();
                return;
            }
            "jie" | "jio" => {
                let (register, offset) = rest.split_once(", ").unwrap();
                let val = if register == "a" { self.a } else { self.b };
                if (val % 2 == 0 && instruction == "jie") || (val == 1 && instruction == "jio") {
                    self.pointer += offset.parse::<i64>().unwrap();
                    return;
                }
            },
            _ => panic!("Unknown instruction {}", instruction)
        }
        self.pointer += 1;
    }

    fn simulate_program(&mut self, program: &Vec<&str>) {
        while self.pointer < program.len() as i64 {
            self.execute_instruction(&program[self.pointer as usize]);
        }
    }

}

pub fn part_one(input: &str) -> Option<i64> {
    let program = input.trim().lines().collect_vec();
    let mut machine = Machine::new();
    machine.simulate_program(&program);
    Some(machine.b)
}

pub fn part_two(input: &str) -> Option<i64> {
    let program = input.trim().lines().collect_vec();
    let mut machine = Machine::new();
    machine.a = 1;
    machine.simulate_program(&program);
    Some(machine.b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
