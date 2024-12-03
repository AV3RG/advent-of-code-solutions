use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sol = input.trim().lines().map(|line| {
        return regex.captures_iter(line).map(|capture| {
            return capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap()
        }).sum::<u32>();
    }).sum();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mult_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let mul_captures = mult_regex.captures_iter(input).collect::<Vec<_>>();
    let do_captures = do_regex.captures_iter(input).map(|capture| {
        capture.get(0).unwrap().start()
    }).collect::<Vec<_>>();
    let dont_captures = dont_regex.captures_iter(input).map(|capture| {
        capture.get(0).unwrap().start()
    }).collect::<Vec<_>>();
    let sol = mul_captures.iter().map(|captures| {
        let mul_start = captures.get(0).unwrap().start();
        let dont_last = dont_captures.iter().map_while(|it| {
            return if it < &mul_start {
                Some(it)
            } else { None }
        }).last();
        let do_last = do_captures.iter().map_while(|it| {
            return if it < &mul_start {
                Some(it)
            } else { None }
        }).last();
        if dont_last == None || (do_last != None && dont_last.unwrap() < do_last.unwrap()) {
            return captures[1].parse::<u32>().unwrap() * captures[2].parse::<u32>().unwrap()
        }
        return 0
    }).sum::<u32>();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
