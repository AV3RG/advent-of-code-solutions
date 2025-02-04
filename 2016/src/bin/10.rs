use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Clone)]
enum TokenReceiver {
    Bot(u16),
    Output(u16)
}

#[derive(Clone)]
struct BotState {
    self_id: u16,
    lower_value: Option<u16>,
    higher_value: Option<u16>,
    give_low_to: Option<TokenReceiver>,
    give_high_to: Option<TokenReceiver>,
    handed: bool
}

impl BotState {
    fn new(self_id: u16) -> BotState {
        BotState {
            self_id,
            lower_value: None,
            higher_value: None,
            give_low_to: None,
            give_high_to: None,
            handed: false
        }
    }

    fn add_incoming(&mut self, value: u16) {
        if self.lower_value.is_none() {
            self.lower_value = Some(value);
        } else {
            if value >= self.lower_value.unwrap() {
                self.higher_value = Some(value);
            } else {
                self.higher_value = self.lower_value.clone();
                self.lower_value = Some(value);
            }
        }
    }

    fn ready_to_perform(&self) -> bool {
        self.lower_value.is_some() && self.higher_value.is_some() && self.give_low_to.is_some() && self.give_high_to.is_some() && !self.handed
    }
}

fn process_bot(bot_id: u16, bot_map: &mut HashMap<u16, BotState>, output_map: &mut HashMap<u16 ,u16>, deque: &mut VecDeque<u16>) {
    let bot = bot_map.get_mut(&bot_id).unwrap();
    bot.handed = true;
    let (lv ,hv) = (bot.lower_value.unwrap(), bot.higher_value.unwrap());
    let (lr, hr) = (bot.clone().give_low_to.unwrap(), bot.clone().give_high_to.unwrap());
    match lr {
        TokenReceiver::Bot(lrb) => {
            let lower = bot_map.get_mut(&lrb).unwrap();
            lower.add_incoming(lv);
            if lower.ready_to_perform() {
                deque.push_back(lower.self_id);
            }
        },
        TokenReceiver::Output(lro) => {
            output_map.insert(lro, lv);
        }
    }
    match hr {
        TokenReceiver::Bot(hrb) => {
            let higher = bot_map.get_mut(&hrb).unwrap();
            higher.add_incoming(hv);
            if higher.ready_to_perform() {
                deque.push_back(higher.self_id);
            }
        },
        TokenReceiver::Output(hro) => {
            output_map.insert(hro, hv);
        }
    }
}

fn process_input(input: &str) -> (HashMap<u16, BotState>, VecDeque<u16>) {
    let mut bots = HashMap::new();
    let mut workable_bots = VecDeque::new();
    input.trim().lines().for_each(|l| {
        let split = l.split_whitespace().collect_vec();
        if split[0] == "value" {
            let (value, bot) = (split[1].parse::<u16>().unwrap(), split[5].parse::<u16>().unwrap());
            let bot = bots.entry(bot).or_insert(BotState::new(bot));
            bot.add_incoming(value);
            if bot.ready_to_perform() {
                workable_bots.push_back(bot.self_id);
            }
        } else if split[0] == "bot" {
            let (bot, low_rec, high_rec) = (split[1].parse::<u16>().unwrap(), split[6].parse::<u16>().unwrap(), split[11].parse::<u16>().unwrap());
            let bot = bots.entry(bot).or_insert(BotState::new(bot));
            bot.give_low_to = Some(if split[5] == "bot" { TokenReceiver::Bot(low_rec) } else { TokenReceiver::Output(low_rec) });
            bot.give_high_to = Some(if split[10] == "bot" { TokenReceiver::Bot(high_rec) } else { TokenReceiver::Output(high_rec) } );
            if bot.ready_to_perform() {
                workable_bots.push_back(bot.self_id);
            }
        }
    });
    (bots, workable_bots)
}

pub fn part_one(input: &str) -> Option<u16> {
    let (mut bots, mut workable_bots) = process_input(input);
    let mut output_bits = HashMap::new();
    loop {
        if workable_bots.is_empty() {
            break;
        }
        let bot_id = workable_bots.pop_front().unwrap();
        let bot = bots.get_mut(&bot_id).unwrap();
        if bot.lower_value.unwrap() == 17 && bot.higher_value.unwrap() == 61 {
            return bot.self_id.into()
        }
        process_bot(bot.self_id, &mut bots, &mut output_bits, &mut workable_bots);
    }
    panic!("No solution found!")
}

pub fn part_two(input: &str) -> Option<u16> {
    let (mut bots, mut workable_bots) = process_input(input);
    let mut output_bits = HashMap::new();
    loop {
        if workable_bots.is_empty() {
            break;
        }
        let bot_id = workable_bots.pop_front().unwrap();
        let bot = bots.get_mut(&bot_id).unwrap();
        process_bot(bot.self_id, &mut bots, &mut output_bits, &mut workable_bots);
    }
    (output_bits[&0] * output_bits[&1] * output_bits[&2]).into()
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
