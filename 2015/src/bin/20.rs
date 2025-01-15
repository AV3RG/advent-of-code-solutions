advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let needed = input.trim().parse::<usize>().unwrap();
    let presents_delivered = 10;
    let cap = needed / presents_delivered;
    let mut array = vec![presents_delivered; cap];
    for i in 2..cap {
        let j = i * presents_delivered;
        let mut k = i;
        while k < cap {
            array[k] += j;
            k += i;
        }
        if array[i] >= needed {
            return Some(i)
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let needed = input.trim().parse::<usize>().unwrap();
    let presents_delivered = 11;
    let cap = needed / presents_delivered;
    let mut array = vec![presents_delivered; cap];
    for i in 2..cap {
        let j = i * presents_delivered;
        let mut k = i;
        let mut delivered = 0;
        while delivered < 50 && k < cap {
            array[k] += j;
            k += i;
            delivered += 1;
        }
        if array[i] >= needed {
            return Some(i)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
