use aoc_utils::grid_utils::{get_manhattan_neighbours, get_similar_region_around};
use aoc_utils::range_utils::convert_to_ranges;

advent_of_code::solution!(12);

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn relative(anchor: (i32, i32), new_pos: (i32, i32)) -> Option<Direction> {
        match (new_pos.0 - anchor.0, new_pos.1 - anchor.1) {
            (0, 1) => Some(Direction::East),
            (0, -1) => Some(Direction::West),
            (1, 0) => Some(Direction::South),
            (-1, 0) => Some(Direction::North),
            _ => { None }
        }
    }
}

#[derive(Debug)]
struct RegionMarkedPlot {
    edges: Vec<Direction>,
    crop: char,
    region: u16
}

fn plot_matcher(plot: &RegionMarkedPlot, other: &RegionMarkedPlot) -> bool {
    plot.crop == other.crop
}

fn process_input(input: &str) -> (Vec<Vec<RegionMarkedPlot>>, usize, usize) {
    let normal_grid=  input.trim().lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (n, m) = (normal_grid.len(), normal_grid[0].len());
    let plot_grid = normal_grid.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, c)| {
            let mut edges = get_manhattan_neighbours((y, x), m, n)
                .iter()
                .filter(|new_pos| {
                    normal_grid[new_pos.0][new_pos.1] != *c
                })
                .map(|new_pos| {
                    Direction::relative((y as i32, x as i32), (new_pos.0 as i32, new_pos.1 as i32)).unwrap()
                })
                .collect::<Vec<_>>();
            if x == 0 {
                edges.push(Direction::West);
            }
            if x == m - 1 {
                edges.push(Direction::East);
            }
            if y == 0 {
                edges.push(Direction::North);
            }
            if y == n - 1 {
                edges.push(Direction::South);
            }
            RegionMarkedPlot { edges, crop: *c, region: 0 }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    (plot_grid, n, m)
}

fn mark_regions(plot_grid: &mut Vec<Vec<RegionMarkedPlot>>, n: usize, m: usize) -> u16 {
    let mut region_marker = 1;
    for y in 0..n {
        for x in 0..m {
            if plot_grid[y][x].region != 0 {
                continue;
            }
            get_similar_region_around(&plot_grid, (y, x), m, n, plot_matcher)
                .iter()
                .for_each(|(ny, nx)| {
                    plot_grid[*ny][*nx].region = region_marker;
                });
            region_marker += 1;
        }
    }
    region_marker
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut plot_grid, n, m) = process_input(input);
    let mut region_marker = mark_regions(&mut plot_grid, n, m);
    let mut sol = 0;
    while region_marker > 0 {
        let mut area_counter = 0;
        let mut edge_counter = 0;
        plot_grid.iter().for_each(|line| {
            line.iter().for_each(|plot| {
                if plot.region == region_marker {
                    area_counter += 1;
                    edge_counter += plot.edges.len() as u64;
                }
            });
        });
        sol += area_counter * edge_counter;
        region_marker -= 1
    }
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut plot_grid, n, m) = process_input(input);
    let mut region_marker = mark_regions(&mut plot_grid, n, m);
    let mut sol = 0;
    while region_marker > 0 {
        let mut area_counter = 0;
        let mut horizontal_line_counter = 0;
        for y in 0..n {
            let matching_horizontal_indices_above = (0..m).filter(|x| {
                plot_grid[y][*x].region == region_marker && (y == 0 || plot_grid[y - 1][*x].region != region_marker)
            }).collect::<Vec<_>>();
            let matching_horizontal_indices_below = (0..m).filter(|x| {
                plot_grid[y][*x].region == region_marker && (y == n - 1 || plot_grid[y + 1][*x].region != region_marker)
            }).collect::<Vec<_>>();
            let ranges_above = convert_to_ranges(&matching_horizontal_indices_above);
            let ranges_below = convert_to_ranges(&matching_horizontal_indices_below);
            horizontal_line_counter += ranges_above.len() as u32 + ranges_below.len() as u32;
        }
        plot_grid.iter().for_each(|line| {
            line.iter().for_each(|plot| {
                if plot.region == region_marker {
                    area_counter += 1;
                }
            });
        });
        sol += area_counter * horizontal_line_counter * 2;
        region_marker -= 1
    }
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
