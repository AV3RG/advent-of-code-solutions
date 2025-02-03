use itertools::Itertools;

advent_of_code::solution!(9);

fn decompress(input: &str, chain: bool) -> String {
    let mut line_pointer = 0;
    let mut result = "".to_string();
    let l = input.chars().collect_vec();
    while line_pointer < l.len() {
        match l[line_pointer] {
            '(' => {
                let mut np = line_pointer + 1;
                while l[np] != ')' { np += 1 }
                let (char_num, rep_time) = &input[(line_pointer + 1)..np].split_once("x").unwrap();
                line_pointer = np + 1;
                let (char_num, rep_time) = (char_num.parse::<usize>().unwrap(), rep_time.parse::<usize>().unwrap());
                if chain {
                    result += &decompress(&input[line_pointer..(line_pointer + char_num)], true).repeat(rep_time);
                } else {
                    result += &*input[line_pointer..(line_pointer + char_num)].repeat(rep_time);
                }
                line_pointer += char_num;
            },
            x => {
                result.push(x);
                line_pointer += 1
            },
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let output = decompress(input.trim(), false);
    Some(output.len())
}



pub fn part_two(input: &str) -> Option<usize> {
    let output = decompress(input.trim(), true);
    Some(output.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 18.into());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 20.into());
    }
}
