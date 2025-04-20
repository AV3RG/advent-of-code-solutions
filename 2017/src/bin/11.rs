advent_of_code::solution!(11);

fn process_move(m: &str, x: &mut f32, y: &mut f32) {
    match m {
        "n" => *y += 1.0,
        "s" => *y -= 1.0,
        "ne" => { *x += 1.0; *y += 0.5 },
        "nw" => { *x -= 1.0; *y += 0.5 },
        "se" => { *x += 1.0; *y -= 0.5 },
        "sw" => { *x -= 1.0; *y -= 0.5 },
        _ => panic!("Invalid move"),
    }
}

pub fn part_one(input: &str) -> Option<f32> {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    input.trim().split(",").for_each(|m| {
        process_move(m, &mut x, &mut y);
    });
    let sol = x.abs() + y.abs() - x.abs() / 2.0;
    Some(sol)
}

pub fn part_two(input: &str) -> Option<f32> {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut max_dist = 0f32;
    input.trim().split(",").for_each(|m| {
        process_move(m, &mut x, &mut y);
        max_dist = max_dist.max(x.abs() + y.abs() - x.abs() / 2.0);
    });
    Some(max_dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3.0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3.0));
    }
}
