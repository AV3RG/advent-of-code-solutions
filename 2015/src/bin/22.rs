use std::cmp::max;
use iter_tools::Itertools;
use pathfinding::prelude::{dijkstra};
use regex::Regex;

advent_of_code::solution!(22);

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct GameState {
    player_turn: bool,
    player_hp: i16,
    player_mana: u16,
    boss_hp: i16,
    shield_timer: u16,
    poison_timer: u16,
    recharge_timer: u16,
    total_spending: u16,
}


impl GameState {

    fn new(boss_hp: i16) -> GameState {
        GameState {
            player_turn: true,
            player_hp: 50,
            player_mana: 500,
            boss_hp,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            total_spending: 0,
        }
    }

    fn advance_game_state(&self, boss_damage: i16, starting_damage: i16) -> Vec<GameState> {
        if self.player_hp <= 0 {
            return vec![];
        }
        let mut base_next = self.clone();
        if base_next.player_turn {
            base_next.player_hp -= starting_damage;
        }
        if base_next.shield_timer > 0 {
            base_next.shield_timer -= 1;
        }
        if base_next.poison_timer > 0 {
            base_next.poison_timer -= 1;
            base_next.boss_hp -= 3;
        }
        if base_next.recharge_timer > 0 {
            base_next.recharge_timer -= 1;
            base_next.player_mana += 101;
        }
        if base_next.boss_hp < 0 {
            return vec![base_next];
        }
        if !base_next.player_turn {
            let mut damage = boss_damage;
            if base_next.shield_timer > 0 {
                damage -= 7;
                damage = max(damage, 1);
            }
            base_next.player_hp -= damage;
            base_next.player_turn = true;
            return vec![base_next];
        }
        base_next.player_turn = false;
        let mut all_next_states: Vec<GameState> = vec![];
        let mut missile = base_next.clone();
        if missile.player_mana >= 53 {
            missile.player_mana -= 53;
            missile.total_spending += 53;
            missile.boss_hp -= 4;
            all_next_states.push(missile);
        }
        let mut drain = base_next.clone();
        if drain.player_mana >= 73 {
            drain.player_mana -= 73;
            drain.total_spending += 73;
            drain.boss_hp -= 2;
            drain.player_hp += 2;
            all_next_states.push(drain);
        }
        let mut shield = base_next.clone();
        if shield.player_mana >= 113 && shield.shield_timer == 0 {
            shield.player_mana -= 113;
            shield.total_spending += 113;
            shield.shield_timer = 6;
            all_next_states.push(shield);
        }
        let mut poison = base_next.clone();
        if poison.player_mana >= 173 && poison.poison_timer == 0 {
            poison.player_mana -= 173;
            poison.total_spending += 173;
            poison.poison_timer = 6;
            all_next_states.push(poison);
        }
        let mut recharge = base_next.clone();
        if recharge.player_mana >= 229 && recharge.recharge_timer == 0 {
            recharge.player_mana -= 229;
            recharge.total_spending += 229;
            recharge.recharge_timer = 5;
            all_next_states.push(recharge);
        }
        all_next_states
    }

}

fn solve(input: &str, starting_damage: i16) -> u32 {
    let regex = Regex::new(r"(\d+)").unwrap();
    let (boss_hp, boss_damage) = regex.captures_iter(input.trim()).map(|captures| {
        captures.get(1).unwrap().as_str().parse::<i16>().unwrap()
    }).collect_tuple().unwrap();
    let start = GameState::new(boss_hp);
    let sol = dijkstra(&start, |start| {
        let vec = start.advance_game_state(boss_damage, starting_damage);
        vec.into_iter().map(|dest| {
            let cost  = dest.total_spending - start.total_spending;
            (dest, cost)
        }).collect_vec().into_iter()
    }, |end| {
        end.boss_hp <= 0
    });
    sol.unwrap().1 as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 0).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 1).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(212));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(212));
    }
}
