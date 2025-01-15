use iter_tools::Itertools;

advent_of_code::solution!(21);

struct Buyable {
    cost: i16,
    damage: i16,
    armor: i16,
}

impl Buyable {
    fn new(cost: i16, damage: i16, armor: i16) -> Buyable {
        Buyable { cost, damage, armor }
    }
}

#[derive(Clone, Debug)]
struct Stats {
    health: u16,
    attack: i16,
    defense: i16,
}

impl Stats {

    const fn new(health: u16, attack: i16, defense: i16) -> Stats {
        Stats { health, attack, defense }
    }

}

fn generate_all_combinations() -> Vec<(i16, Stats)> {
    let weapons: Vec<Buyable> = vec![
        Buyable::new(8, 4, 0),
        Buyable::new(10 ,5, 0),
        Buyable::new(25, 6, 0),
        Buyable::new(40, 7, 0),
        Buyable::new(74, 8, 0),
    ];
    let armors: Vec<Buyable> = vec![
        Buyable::new(0, 0, 0),
        Buyable::new(13, 0, 1),
        Buyable::new(31, 0, 2),
        Buyable::new(53, 0, 3),
        Buyable::new(75, 0, 4),
        Buyable::new(102, 0, 5),
    ];
    let rings: Vec<Buyable> = vec![
        Buyable::new(25, 1, 0),
        Buyable::new(50, 2, 0),
        Buyable::new(100, 3, 0),
        Buyable::new(20, 0, 1),
        Buyable::new(40, 0, 2),
        Buyable::new(80, 0, 3),
    ];
    let mut combinations = Vec::new();
    for weapon in weapons.iter() {
        for armor in armors.iter() {
            combinations.push((
                weapon.cost + armor.cost,
                Stats::new(
                    100,
                    weapon.damage + armor.damage,
                    weapon.armor + armor.armor
                )
            ));
            for ring in rings.iter() {
                combinations.push((
                    weapon.cost + armor.cost + ring.cost,
                    Stats::new(
                        100,
                        weapon.damage + armor.damage + ring.damage,
                        weapon.armor + armor.armor + ring.armor
                    )
                ));
            }
            rings.iter().combinations(2).for_each(|combination| {
                combinations.push((
                    weapon.cost + armor.cost + combination[0].cost + combination[1].cost,
                    Stats::new(
                        100,
                        weapon.damage + armor.damage + combination[0].damage + combination[1].damage,
                        weapon.armor + armor.armor + combination[0].armor + combination[1].armor
                    )
                ))
            })
        }
    }
    combinations
}

fn can_player_win(player_orig: &Stats, villain_orig: &Stats) -> bool {
    let player_moves_needed = villain_orig.health.div_ceil((player_orig.attack - villain_orig.defense).max(1) as u16);
    let villain_moves_needed = player_orig.health.div_ceil((villain_orig.attack - player_orig.defense).max(1) as u16);
    player_moves_needed <= villain_moves_needed
}

fn process_input(input: &str) -> Stats {
    let (hp, damage, armor) = input.trim().lines().map(|line| {
        line.split_once(": ").unwrap().1.parse::<i16>().unwrap()
    }).collect_tuple().unwrap();
    Stats::new(hp as u16, damage, armor)
}

pub fn part_one(input: &str) -> Option<i16> {
    let mut combinations = generate_all_combinations();
    combinations.sort_by_key(|a| a.0);
    let villain = process_input(input);
    combinations.iter().find(|combination| {
        can_player_win(&combination.1, &villain)
    }).unwrap().0.into()
}

pub fn part_two(input: &str) -> Option<i16> {
    let mut combinations = generate_all_combinations();
    combinations.sort_by_key(|a| (a.0 as i16) * -1);
    let villain = process_input(input);
    combinations.iter().find(|combination| {
        !can_player_win(&combination.1, &villain)
    }).unwrap().0.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(121));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(201));
    }
}
