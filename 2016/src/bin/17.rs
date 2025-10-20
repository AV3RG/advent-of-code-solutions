use crypto::digest::Digest;
use crypto::md5::Md5;
use pathfinding::prelude::{bfs, dfs};

advent_of_code::solution!(17);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    current_pos: (i32, i32),
    path: String,
}

fn just_hash(input: &String) -> String {
    let mut md5 = Md5::new();
    md5.input_str(input);
    md5.result_str()
}

fn is_allowed(c: char) -> bool {
    match c {
        'b' | 'c' | 'd' | 'e' | 'f' => true,
        _ => false,
    }
}

impl State {

    fn next_states(&self) -> Vec<State> {
        let mut states = vec![];
        let output = just_hash(&self.path);
        let mut chars = output.chars();
        let top = is_allowed(chars.next().unwrap());
        let bottom = is_allowed(chars.next().unwrap());
        let left = is_allowed(chars.next().unwrap());
        let right = is_allowed(chars.next().unwrap());
        if top && self.current_pos.1 > 0 {
            states.push(self.with_mutation("U", (0, -1)))
        }
        if bottom && self.current_pos.1 < 3 {
            states.push(self.with_mutation("D", (0, 1)))
        }
        if left && self.current_pos.0 > 0 {
            states.push(self.with_mutation("L", (-1, 0)))
        }
        if right && self.current_pos.0 < 3 {
            states.push(self.with_mutation("R", (1, 0)))
        }
        states
    }

    fn is_final(&self) -> bool {
        self.current_pos.0 == 3 && self.current_pos.1 == 3
    }

    fn with_mutation(&self, path_add: &str, pos_change: (i32, i32)) -> Self {
        let mut new = self.clone();
        new.path += path_add;
        new.current_pos.0 += pos_change.0;
        new.current_pos.1 += pos_change.1;
        new
    }

}

pub fn part_one(input: &str) -> Option<String> {
    let input = input.trim().to_string();
    let start_state = State {
        current_pos: (0, 0),
        path: input.clone(),
    };
    let option = bfs(&start_state, |curr| curr.next_states(), |poss| poss.is_final());
    Some(option.unwrap().last().unwrap().path.replace(&input, ""))
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim().to_string();
    let start_state = State {
        current_pos: (0, 0),
        path: input.clone(),
    };
    let mut longest = 0;
    dfs(start_state, |curr| {
        if curr.is_final() { vec![] } else { curr.next_states() }
    }, |poss| {
        if poss.is_final() && poss.path.len() > longest {
            longest = poss.path.len();
        }
        false
    });
    Some(longest - input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("DDRRRD".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(370));
    }
}
