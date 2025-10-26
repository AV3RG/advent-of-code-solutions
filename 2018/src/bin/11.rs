advent_of_code::solution!(11);

struct SummedAreaTable {
    storage: Vec<i32>,
}

impl SummedAreaTable {
    const SIZE: i32 = 300;

    fn new(serial_number: i32) -> Self {
        let mut result = Self {
            storage: vec![0; (Self::SIZE * Self::SIZE) as usize],
        };
        for y in 1..=Self::SIZE {
            let mut row_value: i32 = 0;
            for x in 1..=Self::SIZE {
                row_value += calculate_power(serial_number, x, y);
                let storage_index = x - 1 + (y - 1) * Self::SIZE;
                result.storage[storage_index as usize] = row_value + result.at(x, y - 1);
            }
        }
        result
    }

    fn at(&self, x: i32, y: i32) -> i32 {
        if x < 1 || y < 1 {
            return 0;
        }
        self.storage
            .get((x - 1 + (y - 1) * Self::SIZE) as usize)
            .copied()
            .unwrap_or(0)
    }

    fn square_power(&self, x: i32, y: i32, size: i32) -> i32 {
        self.at(x - 1, y - 1) + self.at(x + size - 1, y + size - 1)
            - self.at(x + size - 1, y - 1)
            - self.at(x - 1, y + size - 1)
    }
}

fn calculate_power(serial_number: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level -= 5;
    power_level
}

pub fn part_one(input: &str) -> Option<String> {
    let grid_serial_number = input.trim().parse::<i32>().unwrap();
    let mut fuel_grid = Vec::new();
    for x in 1..=300 {
        let mut current_row = Vec::new();
        for y in 1..=300 {
            current_row.push(calculate_power(x, y, grid_serial_number));
        }
        fuel_grid.push(current_row);
    }
    let mut max_power = 0;
    let mut coordinates = None;
    for x in 0..=297 {
        for y in 0..=297 {
            let current_power =
                fuel_grid[x][y] + fuel_grid[x + 1][y] + fuel_grid[x + 2][y] +
                fuel_grid[x][y + 1] + fuel_grid[x + 1][y + 1] + fuel_grid[x + 2][y + 1] +
                fuel_grid[x][y + 2] + fuel_grid[x + 1][y + 2] + fuel_grid[x + 2][y + 2];
            if current_power > max_power {
                max_power = current_power;
                coordinates = Some((x + 1, y + 1));
            }
        }
    };
    Some(coordinates.unwrap().0.to_string() + "," + &*coordinates.unwrap().1.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let grid_serial_number = input.trim().parse::<i32>().unwrap();
    let table = SummedAreaTable::new(grid_serial_number);

    let mut max_power = 0;
    let mut max_square_width = 0;
    let mut max_point = (0, 0);

    for square_width in 1..=300 {
        for y in 1..=(300 - square_width) {
            for x in 1..=(300 - square_width) {
                let square_power = table.square_power(x, y, square_width);
                if square_power > max_power {
                    max_square_width = square_width;
                    max_power = square_power;
                    max_point = (x, y);
                }
            }
        }
    }

    let result = max_point.0.to_string() + "," + &*max_point.1.to_string() + "," + &*max_square_width.to_string();
    Some(result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("94,27".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("135,154,12".to_string()));
    }
}
