advent_of_code::solution!(17);

#[derive(Clone, Copy, Debug)]
struct MachineState {
    instruction_pointer: usize,
    registers: RegisterSet
}

#[derive(Clone, Copy, Debug)]
struct RegisterSet {
    a: u64,
    b: u64,
    c: u64
}

impl MachineState {

    fn run_program(&mut self, program: &Vec<u64>) -> Vec<u64> {
        let mut output = Vec::new();
        while self.instruction_pointer < program.len() {
            let opcode = program.get(self.instruction_pointer).unwrap();
            let operand = program.get(self.instruction_pointer + 1).unwrap();
            let o = self.process_operation(*opcode, *operand);
            if o.is_some() {
                output.push(o.unwrap());
            }
        };
        output
    }

    fn process_operation(&mut self, opcode: u64, operand: u64) -> Option<u64> {
        return match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid op code")
        }
    }

    fn combo_operand_value(&self, operand: u64) -> u64 {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            _ => panic!("Unexpected combo operand")
        }
    }

    fn reset_machine(&mut self) {
        self.instruction_pointer = 0;
        self.registers.a = 0;
        self.registers.b = 0;
        self.registers.c = 0;
    }

    fn advance_instruction(&mut self) {
        self.instruction_pointer += 2;
    }

    fn adv(&mut self, operand: u64) -> Option<u64> {
        let operand_value = self.combo_operand_value(operand);
        self.registers.a = (self.registers.a / 2u64.pow(operand_value as u32)) as u64;
        self.advance_instruction();
        None
    }

    fn bxl(&mut self, operand_value: u64) -> Option<u64> {
        self.registers.b = self.registers.b ^ operand_value;
        self.advance_instruction();
        None
    }

    fn bst(&mut self, operand: u64) -> Option<u64> {
        let operand_value = self.combo_operand_value(operand);
        self.registers.b = operand_value % 8;
        self.advance_instruction();
        None
    }

    fn jnz(&mut self, operand_value: u64) -> Option<u64> {
        if self.registers.a != 0 {
            self.instruction_pointer = operand_value as usize
        } else {
            self.advance_instruction();
        }
        None
    }

    fn bxc(&mut self, _ignored: u64) -> Option<u64> {
        self.registers.b = self.registers.b ^ self.registers.c;
        self.advance_instruction();
        None
    }

    fn out(&mut self, operand: u64) -> Option<u64> {
        self.advance_instruction();
        return Some(self.combo_operand_value(operand) % 8);
    }

    fn bdv(&mut self, operand: u64) -> Option<u64> {
        let operand_value = self.combo_operand_value(operand);
        self.registers.b = (self.registers.a / 2u64.pow(operand_value as u32)) as u64;
        self.advance_instruction();
        None
    }

    fn cdv(&mut self, operand: u64) -> Option<u64> {
        let operand_value = self.combo_operand_value(operand);
        self.registers.c = (self.registers.a / 2u64.pow(operand_value as u32)) as u64;
        self.advance_instruction();
        None
    }

}

fn process_input(input: &str) -> (MachineState, Vec<u64>) {
    let (reg_string, program_str) = input.trim().split_once("\n\n").unwrap();
    let mut register_values = reg_string.trim().splitn(3, "\n").map(|a| {
        a.split_once(": ").unwrap().1.parse::<u64>().unwrap()
    }).collect::<Vec<u64>>().into_iter();
    let register_set = RegisterSet {
        a: register_values.next().unwrap(),
        b: register_values.next().unwrap(),
        c: register_values.next().unwrap()
    };
    let machine = MachineState {
        instruction_pointer: 0,
        registers: register_set
    };
    let program = program_str.trim()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|i| {
            i.parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();
    (machine, program)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut machine, program) = process_input(input);
    let output = machine.run_program(&program);
    Some(output.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut machine, program) = process_input(input);
    let mut a = 0;
    let mut skipper = program.len() - 1;
    while skipper > 0 {
        a <<= 3;
        machine.reset_machine();
        machine.registers.a = a;
        while machine.run_program(&program) != program[(skipper - 1)..] {
            a += 1;
            machine.reset_machine();
            machine.registers.a = a;
        }
        skipper -= 1;
    };
    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(117440));
    }
}
