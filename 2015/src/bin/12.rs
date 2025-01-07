use regex::Regex;
use serde_json::Value;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i32> {
    let regex = Regex::new(r"(?P<number_with_sign>-?\d+)").unwrap();
    regex.find_iter(input.trim()).into_iter().map(|captured| {
        captured.as_str().parse::<i32>().unwrap()
    }).sum::<i32>().into()
}

fn calculate_value(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Object(o) => {
            if o.values().any(|v| v.is_string() && v.as_str().unwrap() == "red") { 0 } else { o.values().map(|v| calculate_value(v)).sum() }
        },
        Value::Array(a) => a.iter().map(|v| calculate_value(v)).sum(),
        _ => 0,
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let parsed = serde_json::from_str::<Value>(input.trim()).unwrap();
    calculate_value(&parsed).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(4));
    }
}
