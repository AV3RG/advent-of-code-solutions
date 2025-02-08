use itertools::Itertools;

advent_of_code::solution!(12);

fn register_index(register: &str) -> usize {
    ((register.parse::<char>().unwrap() as u8) - ('a' as u8)) as usize
}

fn process_instruction(instruction: &str, machine_state: &mut Vec<i32>, instruction_pointer: &mut i32) {
    let parts = instruction.split_whitespace().collect_vec();
    match parts[0] {
        "cpy" => {
            let parse_try = parts[1].parse::<i32>();
            if parse_try.is_err() {
                machine_state[register_index(parts[2])] = machine_state[register_index(parts[1])]
            } else {
                machine_state[register_index(parts[2])] = parse_try.unwrap();
            }
        },
        "inc" => {
            machine_state[register_index(parts[1])] += 1
        },
        "dec" => {
            machine_state[register_index(parts[1])] -= 1
        },
        "jnz" => {
            let parse_try = parts[1].parse::<i32>();
            if (parse_try.is_err() && machine_state[register_index(parts[1])] != 0) || (parse_try.is_ok() && parse_try.unwrap() != 0) {
                *instruction_pointer += parts[2].parse::<i32>().unwrap();
                return;
            }
        },
        _ => panic!("Unknown instruction: {}", instruction),
    }
    *instruction_pointer += 1;
}

fn run_simulation(input: &str, machine_state: &mut Vec<i32>) {
    let mut instruction_pointer = 0i32;
    let program = input.trim().lines().collect_vec();
    while instruction_pointer < program.len() as i32 {
        process_instruction(program[instruction_pointer as usize], machine_state, &mut instruction_pointer);
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut machine_state = vec![0; 4];
    run_simulation(input, &mut machine_state);
    Some(machine_state[0])
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut machine_state = vec![0, 0, 1, 0];
    run_simulation(input, &mut machine_state);
    Some(machine_state[0] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
