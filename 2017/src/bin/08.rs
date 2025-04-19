use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;

advent_of_code::solution!(8);

enum Operation {
    Increment,
    Decrement,
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Operation::Increment),
            "dec" => Ok(Operation::Decrement),
            _ => Err(format!("Unknown operation: {}", s)),
        }
    }
}

enum CheckingOperation {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
}

impl FromStr for CheckingOperation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(CheckingOperation::GreaterThan),
            "<" => Ok(CheckingOperation::LessThan),
            ">=" => Ok(CheckingOperation::GreaterThanEqual),
            "<=" => Ok(CheckingOperation::LessThanEqual),
            "==" => Ok(CheckingOperation::Equal),
            "!=" => Ok(CheckingOperation::NotEqual),
            _ => Err(format!("Unknown operation: {}", s)),
        }
    }
}

struct Condition {
    register: String,
    checking_operation: CheckingOperation,
    value: i32,
}

impl FromStr for Condition {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect_vec();
        Ok(Condition {
            register: split[0].to_string(),
            checking_operation: split[1].parse()?,
            value: split[2].parse().unwrap(),
        })
    }
}

impl Condition {
    fn check(&self, registers: &HashMap<String, i32>) -> bool {
        let register_value = registers.get(&self.register).unwrap_or(&0);
        match self.checking_operation {
            CheckingOperation::Equal => register_value == &self.value,
            CheckingOperation::NotEqual => register_value != &self.value,
            CheckingOperation::GreaterThan => register_value > &self.value,
            CheckingOperation::GreaterThanEqual => register_value >= &self.value,
            CheckingOperation::LessThan => register_value < &self.value,
            CheckingOperation::LessThanEqual => register_value <= &self.value,
        }
    }
}

struct ConditionedExpression {
    register: String,
    operation: Operation,
    value: i32,
    condition: Condition
}

impl FromStr for ConditionedExpression {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.splitn(5, " ").collect_vec();
        Ok(ConditionedExpression {
            register: split[0].to_string(),
            operation: split[1].parse()?,
            value: split[2].parse().unwrap(),
            condition: split[4].parse()?
        })
    }
}

impl ConditionedExpression {
    fn execute(&self, registers: &mut HashMap<String, i32>) -> Option<i32> {
        if !self.condition.check(registers) { return None; }
        match self.operation {
            Operation::Increment => { *registers.entry(self.register.clone()).or_insert(0) += self.value }
            Operation::Decrement => { *registers.entry(self.register.clone()).or_insert(0) -= self.value }
        }
        Some(*registers.get(&self.register).unwrap_or(&0))
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let vec = input
        .trim()
        .lines()
        .map(|line| line.parse::<ConditionedExpression>().unwrap())
        .collect_vec();
    let mut registers = HashMap::new();
    vec.iter().for_each(|e| {
        e.execute(&mut registers);
    });
    registers.values().max().cloned()
}

pub fn part_two(input: &str) -> Option<i32> {
    let vec = input
        .trim()
        .lines()
        .map(|line| line.parse::<ConditionedExpression>().unwrap())
        .collect_vec();
    let mut registers = HashMap::new();
    let mut max_value = 0;
    vec.iter().for_each(|e| {
        let option = e.execute(&mut registers);
        if let Some(value) = option {
            max_value = value.max(max_value);
        }
    });
    Some(max_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
