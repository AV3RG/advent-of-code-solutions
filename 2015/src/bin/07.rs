use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
enum ConnectionInput {
    Value(u16),
    Connection(String)
}

impl ConnectionInput {
    fn from_str(s: &str) -> ConnectionInput {
        let parse_result = s.parse::<u16>();
        let input = if parse_result.is_ok() { ConnectionInput::Value(parse_result.unwrap()) } else { ConnectionInput::Connection(s.to_string()) };
        input
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    And,
    Or,
    LShift(u16),
    RShift(u16),
    Not,
    Direct
}

#[derive(Debug, Clone)]
struct Connection {
    input1: ConnectionInput,
    input2: Option<ConnectionInput>,
    operation: Operation,
    output: String
}

impl Connection {

    fn feasible(&self, map: &HashMap<String, u16>) -> bool {
        let input1_feasible = match &self.input1 {
            ConnectionInput::Value(_) => true,
            ConnectionInput::Connection(con) => map.contains_key(con)
        };
        let input2_feasible = match &self.input2 {
            Some(ConnectionInput::Value(_)) => true,
            Some(ConnectionInput::Connection(con)) => map.contains_key(con),
            None => true
        };
        input1_feasible && input2_feasible
    }

    fn feed_output(&self, map: &mut HashMap<String, u16>) {
        let input_1_val = match &self.input1 {
            ConnectionInput::Value(val) => *val,
            ConnectionInput::Connection(con) => map.get(con).unwrap().clone()
        };
        let input_2_val = match &self.input2 {
            Some(ConnectionInput::Value(val)) => *val,
            Some(ConnectionInput::Connection(con)) => map.get(con).unwrap().clone(),
            None => 0
        };
        let result = match self.operation {
            Operation::And => input_1_val & input_2_val,
            Operation::Or => input_1_val | input_2_val,
            Operation::LShift(val) => input_1_val << val,
            Operation::RShift(val) => input_1_val >> val,
            Operation::Not => !input_1_val,
            Operation::Direct => input_1_val
        };
        map.insert(self.output.clone(), result);
    }
}

fn process_input(s: &str) -> Vec<Connection> {
    s.trim().lines().map(|line| {
        let (input_part, output_part) = line.trim().split_once(" -> ").unwrap();
        let output = output_part.to_string();
        let input_parts = input_part.split_whitespace().collect::<Vec<_>>();
        return if input_parts.len() == 1 {
            let input = ConnectionInput::from_str(input_parts[0]);
            Connection {
                input1: input,
                input2: None,
                operation: Operation::Direct,
                output
            }
        } else if input_parts.len() == 2 {
            Connection {
                input1: ConnectionInput::Connection(input_parts[1].to_string()),
                input2: None,
                operation: Operation::Not,
                output
            }
        } else if input_parts.len() == 3 {
            let input_1 = ConnectionInput::from_str(input_parts[0]);
            let input_2 = ConnectionInput::from_str(input_parts[2]);
            let input_2_as_val = match input_2 {
                ConnectionInput::Value(val) => Some(val),
                _ => None
            };
            let operation = match input_parts[1] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "LSHIFT" => Operation::LShift(input_2_as_val.unwrap()),
                "RSHIFT" => Operation::RShift(input_2_as_val.unwrap()),
                _ => panic!("Unexpected operation")
            };
            Connection {
                input1: input_1,
                input2: Some(input_2),
                operation,
                output
            }
        } else {
            panic!("Unexpected input")
        };
    }).collect::<Vec<_>>()
}

fn simulate_circuit(mut connections: Vec<Connection>) -> HashMap<String, u16> {
    let mut map = HashMap::new();
    while !connections.is_empty() {
        let mut feasible_connections = Vec::new();
        let mut infeasible_connections = Vec::new();
        connections.iter().for_each(|connection| {
            if connection.feasible(&map) {
                feasible_connections.push(connection);
            } else {
                infeasible_connections.push(connection.clone());
            }
        });
        feasible_connections.iter().for_each(|connection| {
            connection.feed_output(&mut map);
        });
        connections = infeasible_connections;
    };
    map
}

pub fn part_one(input: &str) -> Option<u16> {
    let connections = process_input(input);
    let map = simulate_circuit(connections);
    Some(*map.get("a").unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut connections = process_input(input);
    let map = simulate_circuit(connections.clone());
    let a_output = *map.get("a").unwrap();
    connections.iter_mut().for_each(|connection| {
        if connection.output == "b" {
            connection.input1 = ConnectionInput::Value(a_output);
            connection.input2 = None;
            connection.operation = Operation::Direct;
        }
    });
    let map = simulate_circuit(connections);
    Some(*map.get("a").unwrap() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
